export type ProviderId = "claude" | "codex" | "cursor" | "gemini";

export type MetricType = "percentage" | "tokens" | "requests" | "cost";

export interface ProviderUsage {
  id: ProviderId;
  name: string;
  percentage: number; // 0-100
  usedDisplay: string;
  limitDisplay: string;
  status: "ok" | "warning" | "error" | "offline";
  isEstimate: boolean;
  statusMessage?: string;
  lastUpdated: string;
}

export interface HudConfig {
  position: "top" | "bottom" | "left" | "right";
  isExpanded: boolean;
  theme: "dark" | "oled" | "system";
  opacity: number;
}
