# CodeHalo Architecture Specification

## 1. Overview
**CodeHalo** is a lightweight, cross-platform (Windows & Linux) desktop overlay for monitoring AI coding agent usage (e.g., Claude Code, Codex, Cursor, Gemini/Antigravity).

It sits unobtrusively on the screen edge (Top, Bottom, Left, Right) as a compact HUD, expanding on interaction to provide real-time quota, rate limit, and session visibility without opening multiple vendor dashboards.

---

## 2. Layered Architecture

```text
┌─────────────────────────────────────────────────────────┐
│                       UI Layer                          │
│          TypeScript / HTML / Modern CSS                 │
│  - Compact pill / HUD edge overlay                      │
│  - Expanded multi-provider drawer / card                │
│  - Subtle animations, themes, DPI/scaling adaptivity    │
├─────────────────────────────────────────────────────────┤
│                   Application Layer                     │
│  - Window state (collapsed / expanded / reposition)     │
│  - Event dispatching & frontend Tauri IPC               │
│  - Settings & configuration state management            │
├─────────────────────────────────────────────────────────┤
│                      Core Layer                         │
│  - Provider Registry & Scheduler                        │
│  - Usage normalization & cache                          │
│  - Refresh intervals, rate limit backoff                │
│  - Secure Credential Store (OS Keychain / DPAPI)        │
│  - SQLite local storage (history & logs)                │
├─────────────────────────────────────────────────────────┤
│                Platform Abstraction Layer               │
│  - Window positioning, always-on-top, transparency      │
│  - Multi-monitor discovery, DPI & bounds tracking       │
│  - System tray & global hotkeys                         │
├────────────────────────────┬────────────────────────────┤
│          Windows           │           Linux            │
│   Win32 / DWM / DPAPI      │   Wayland Layer Shell /    │
│   Credential Manager       │   X11 XLib / SecretService │
└────────────────────────────┴────────────────────────────┘
```

---

## 3. Provider Contract & Data Flow

Provider integrations must never touch the UI directly. All providers adhere to a unified trait/interface:

```text
Provider (Fetch/Cache) ──▶ Normalized UsageSnapshot ──▶ Core State ──▶ Tauri Event/IPC ──▶ Overlay UI
```

### Core Provider Interface (Conceptual Rust Trait)

```rust
#[async_trait]
pub trait UsageProvider: Send + Sync {
    /// Unique provider identifier (e.g., "claude", "cursor", "codex")
    fn id(&self) -> &'static str;

    /// Human-friendly provider name
    fn name(&self) -> &'static str;

    /// Icon identifier or SVG
    fn icon(&self) -> &'static str;

    /// Check authentication status & configuration
    async fn status(&self) -> ProviderStatus;

    /// Fetch latest usage snapshots
    async fn get_usage(&self) -> Result<UsageSnapshot, ProviderError>;

    /// Fetch quotas or plan limits
    async fn get_limits(&self) -> Result<PlanLimits, ProviderError>;
}
```

### Usage Transparency Rules
- **Official**: Directly provided by verified provider API/CLI token endpoint.
- **Derived/Local**: Parsed from local session files, CLI sqlite/json caches.
- **Estimated**: Calculated based on tracked tokens / time-window algorithms.
- **Unavailable**: Stored/shown with clear warning badge; **never zeroed out deceptively**.

---

## 4. Platform Abstraction Boundaries

### Windows
- Windows 10 & 11 transparency (`decorations: false`, `transparent: true`, `always_on_top: true`).
- System tray with quick actions (Toggle Overlay, Settings, Quit).
- DPI-aware edge positioning calculated from monitor work areas via Tauri monitor APIs.

### Linux (Wayland & X11)
- Abstracted window positioning and edge anchoring.
- Graceful fallback: Wayland compositors (via layer shell protocols where supported or frameless floating utility) and X11 (`_NET_WM_STATE_STAYS_ON_TOP`, `_NET_WM_WINDOW_TYPE_DOCK` / utility).
- Secret Service integration for credential storage.

---

## 5. Security & Privacy
1. **Local-first**: No telemetry, no external central server, no relaying of credentials.
2. **Credential Isolation**: Uses native OS keyrings (Windows DPAPI / Credential Manager, Linux Secret Service).
3. **Respectful Polling**: Intelligent adaptive intervals to prevent rate limiting and unnecessary battery/CPU wakeups.
