use crate::core::errors::ProviderError;
use crate::core::models::{ProviderId, ProviderSnapshot};
use crate::services::usage_service::UsageService;
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Configuration parameters for provider refresh pacing and backoff.
#[derive(Debug, Clone)]
pub struct RefreshPolicy {
    /// Nominal interval between successful refreshes (default 5 minutes).
    pub default_interval: Duration,
    /// Absolute minimum time between consecutive requests to prevent API spam (default 15 seconds).
    pub min_interval: Duration,
    /// Maximum ceiling for backoff delay on recurring failures (default 10 minutes).
    pub max_backoff: Duration,
    /// Multiplier applied to backoff interval per consecutive failure.
    pub backoff_factor: f64,
}

impl Default for RefreshPolicy {
    fn default() -> Self {
        Self {
            default_interval: Duration::minutes(5),
            min_interval: Duration::seconds(15),
            max_backoff: Duration::minutes(10),
            backoff_factor: 1.5,
        }
    }
}

/// Tracks the refresh cadence, backoff state, and history for a specific provider.
#[derive(Debug, Clone)]
pub struct ProviderSchedule {
    pub last_attempt: Option<DateTime<Utc>>,
    pub last_success: Option<DateTime<Utc>>,
    pub consecutive_failures: u32,
    pub current_interval: Duration,
}

impl ProviderSchedule {
    pub fn new(default_interval: Duration) -> Self {
        Self {
            last_attempt: None,
            last_success: None,
            consecutive_failures: 0,
            current_interval: default_interval,
        }
    }

    pub fn is_due(&self, now: DateTime<Utc>) -> bool {
        match self.last_attempt {
            None => true,
            Some(last) => (now - last) >= self.current_interval,
        }
    }

    pub fn can_force_refresh(&self, now: DateTime<Utc>, min_interval: Duration) -> bool {
        match self.last_attempt {
            None => true,
            Some(last) => (now - last) >= min_interval,
        }
    }

    pub fn record_success(&mut self, now: DateTime<Utc>, default_interval: Duration) {
        self.last_attempt = Some(now);
        self.last_success = Some(now);
        self.consecutive_failures = 0;
        self.current_interval = default_interval;
    }

    pub fn record_failure(&mut self, now: DateTime<Utc>, policy: &RefreshPolicy) {
        self.last_attempt = Some(now);
        self.consecutive_failures = self.consecutive_failures.saturating_add(1);

        let factor = policy.backoff_factor.powi(self.consecutive_failures as i32);
        let base_secs = policy.default_interval.num_seconds() as f64;
        let new_secs = (base_secs * factor) as i64;
        let max_secs = policy.max_backoff.num_seconds();

        self.current_interval = Duration::seconds(new_secs.min(max_secs));
    }
}

/// Service managing periodic, throttled, and backoff-aware polling across providers.
pub struct RefreshService {
    usage_service: Arc<UsageService>,
    policy: RefreshPolicy,
    schedules: Arc<RwLock<HashMap<ProviderId, ProviderSchedule>>>,
}

impl RefreshService {
    pub fn new(usage_service: Arc<UsageService>, policy: RefreshPolicy) -> Self {
        Self {
            usage_service,
            policy,
            schedules: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn with_default_policy(usage_service: Arc<UsageService>) -> Self {
        Self::new(usage_service, RefreshPolicy::default())
    }

    pub fn policy(&self) -> &RefreshPolicy {
        &self.policy
    }

    pub fn usage_service(&self) -> Arc<UsageService> {
        Arc::clone(&self.usage_service)
    }

    /// Checks whether a given provider is currently due for an automated refresh.
    pub fn is_due(&self, id: &ProviderId) -> bool {
        let now = Utc::now();
        let schedules = match self.schedules.read() {
            Ok(s) => s,
            Err(_) => return true,
        };
        match schedules.get(id) {
            Some(schedule) => schedule.is_due(now),
            None => true,
        }
    }

    /// Checks whether a manual force-refresh is permitted by the rate limit.
    pub fn can_force_refresh(&self, id: &ProviderId) -> bool {
        let now = Utc::now();
        let schedules = match self.schedules.read() {
            Ok(s) => s,
            Err(_) => return true,
        };
        match schedules.get(id) {
            Some(schedule) => schedule.can_force_refresh(now, self.policy.min_interval),
            None => true,
        }
    }

    /// Returns a copy of the current schedule metrics for a provider, if initialized.
    pub fn schedule_for(&self, id: &ProviderId) -> Option<ProviderSchedule> {
        self.schedules.read().ok()?.get(id).cloned()
    }

    /// Runs a single tick cycle, refreshing all providers that are currently due.
    pub async fn tick(&self) -> Vec<(ProviderId, Result<ProviderSnapshot, ProviderError>)> {
        let providers = self.usage_service.registry().all();
        let now = Utc::now();
        let mut results = Vec::new();

        for provider in providers {
            let id = provider.id();
            let is_due = {
                let mut schedules = match self.schedules.write() {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                let schedule = schedules
                    .entry(id.clone())
                    .or_insert_with(|| ProviderSchedule::new(self.policy.default_interval));
                schedule.is_due(now)
            };

            if is_due {
                let result = self.usage_service.refresh_provider(&id).await;
                if let Ok(mut schedules) = self.schedules.write() {
                    if let Some(schedule) = schedules.get_mut(&id) {
                        match &result {
                            Ok(_) => schedule.record_success(Utc::now(), self.policy.default_interval),
                            Err(_) => schedule.record_failure(Utc::now(), &self.policy),
                        }
                    }
                }
                results.push((id, result));
            }
        }

        results
    }

    /// Manually triggers a refresh for a provider, enforcing rate limiting.
    pub async fn force_refresh(&self, id: &ProviderId) -> Result<ProviderSnapshot, ProviderError> {
        let now = Utc::now();
        {
            let mut schedules = self
                .schedules
                .write()
                .map_err(|_| ProviderError::Internal(id.clone(), "Lock poisoned".into()))?;
            let schedule = schedules
                .entry(id.clone())
                .or_insert_with(|| ProviderSchedule::new(self.policy.default_interval));

            if !schedule.can_force_refresh(now, self.policy.min_interval) {
                return Err(ProviderError::Internal(id.clone(), "Refresh rate limit exceeded".into()));
            }
        }

        let result = self.usage_service.refresh_provider(id).await;
        if let Ok(mut schedules) = self.schedules.write() {
            if let Some(schedule) = schedules.get_mut(id) {
                match &result {
                    Ok(_) => schedule.record_success(Utc::now(), self.policy.default_interval),
                    Err(_) => schedule.record_failure(Utc::now(), &self.policy),
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::state::AppState;
    use crate::providers::registry::ProviderRegistry;

    #[test]
    fn test_schedule_pacing_and_backoff() {
        let policy = RefreshPolicy::default();
        let mut schedule = ProviderSchedule::new(policy.default_interval);
        let now = Utc::now();

        // Brand new schedule is due immediately
        assert!(schedule.is_due(now));
        assert!(schedule.can_force_refresh(now, policy.min_interval));

        // After failure, backoff increases
        schedule.record_failure(now, &policy);
        assert_eq!(schedule.consecutive_failures, 1);
        assert!(schedule.current_interval > policy.default_interval);
        assert!(!schedule.is_due(now));

        // After success, interval resets to default
        schedule.record_success(now, policy.default_interval);
        assert_eq!(schedule.consecutive_failures, 0);
        assert_eq!(schedule.current_interval, policy.default_interval);
    }

    #[test]
    fn test_refresh_service_tick() {
        let registry = Arc::new(ProviderRegistry::with_defaults());
        let state = Arc::new(AppState::new());
        let usage_service = Arc::new(UsageService::new(registry, state));
        let refresh_service = RefreshService::with_default_policy(usage_service);

        // First tick should trigger refresh for due providers
        let results = futures::executor::block_on(refresh_service.tick());
        assert!(!results.is_empty());

        // Immediately after, providers should NOT be due
        assert!(!refresh_service.is_due(&ProviderId::Claude));
    }
}
