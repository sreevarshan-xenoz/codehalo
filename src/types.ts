export interface MonitorInfo {
  index: usize;
  name: string | null;
  width: number;
  height: number;
  scale_factor: number;
  is_primary: boolean;
}

export type usize = number;

export type EdgePosition = "top" | "bottom" | "left" | "right";

export interface HudWindowConfig {
  edge: EdgePosition;
  monitor_index: number;
  offset_px: number;
}
