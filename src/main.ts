import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import type { EdgePosition, HudWindowConfig, MonitorInfo } from "./types";

const appWindow = getCurrentWindow();

let monitors: MonitorInfo[] = [];
let currentConfig: HudWindowConfig = {
  edge: "top",
  monitor_index: 0,
  offset_px: 12,
};
let isExpanded = false;

async function refreshMonitors() {
  try {
    monitors = await invoke<MonitorInfo[]>("get_monitors");
    renderMonitors();
  } catch (err) {
    console.error("Failed to load monitors", err);
  }
}

async function loadConfig() {
  try {
    currentConfig = await invoke<HudWindowConfig>("get_hud_config");
    updateEdgeButtons();
    updatePillDisplay();
  } catch (err) {
    console.error("Failed to load config", err);
  }
}

function renderMonitors() {
  const select = document.getElementById("monitor-select") as HTMLSelectElement | null;
  const scaleText = document.getElementById("stat-scale-factor");
  if (!select) return;

  if (monitors.length === 0) {
    select.innerHTML = '<option value="0">Default Display (100%)</option>';
    return;
  }

  select.innerHTML = monitors
    .map((m) => {
      const label = `${m.name || `Display ${m.index + 1}`} (${m.width}x${m.height} @ ${Math.round(m.scale_factor * 100)}%)${m.is_primary ? " [Primary]" : ""}`;
      return `<option value="${m.index}" ${m.index === currentConfig.monitor_index ? "selected" : ""}>${label}</option>`;
    })
    .join("");

  const activeMonitor = monitors[currentConfig.monitor_index] || monitors[0];
  if (activeMonitor && scaleText) {
    scaleText.textContent = `${activeMonitor.scale_factor}x (${Math.round(activeMonitor.scale_factor * 100)}%)`;
  }
}

function updateEdgeButtons() {
  const buttons = document.querySelectorAll<HTMLButtonElement>(".btn-edge");
  buttons.forEach((btn) => {
    if (btn.dataset.edge === currentConfig.edge) {
      btn.classList.add("active");
    } else {
      btn.classList.remove("active");
    }
  });
}

function updatePillDisplay() {
  const pillEdgeTag = document.getElementById("pill-edge-tag");
  if (pillEdgeTag) {
    pillEdgeTag.textContent = currentConfig.edge.toUpperCase();
  }
}

async function setExpanded(expanded: boolean) {
  isExpanded = expanded;
  const root = document.getElementById("hud-root");
  if (!root) return;

  if (isExpanded) {
    root.classList.add("expanded");
    await appWindow.setSize(new LogicalSize(380, 260));
  } else {
    root.classList.remove("expanded");
    await appWindow.setSize(new LogicalSize(380, 48));
  }
}

async function setEdgeAndMonitor(edge: EdgePosition, monitorIndex: number) {
  currentConfig.edge = edge;
  currentConfig.monitor_index = monitorIndex;
  updateEdgeButtons();
  updatePillDisplay();

  try {
    await invoke("update_hud_position", {
      edge,
      monitorIndex,
    });
  } catch (err) {
    console.error("Failed to update position:", err);
  }
}

async function init() {
  await refreshMonitors();
  await loadConfig();

  const pill = document.getElementById("hud-pill");
  const btnCollapse = document.getElementById("btn-collapse");
  const monitorSelect = document.getElementById("monitor-select") as HTMLSelectElement | null;
  const edgeButtons = document.querySelectorAll<HTMLButtonElement>(".btn-edge");

  pill?.addEventListener("click", () => {
    setExpanded(!isExpanded);
  });

  btnCollapse?.addEventListener("click", (e) => {
    e.stopPropagation();
    setExpanded(false);
  });

  monitorSelect?.addEventListener("change", async (e) => {
    const target = e.target as HTMLSelectElement;
    const monitorIdx = parseInt(target.value, 10);
    await setEdgeAndMonitor(currentConfig.edge, monitorIdx);
    renderMonitors();
  });

  edgeButtons.forEach((btn) => {
    btn.addEventListener("click", async (e) => {
      e.stopPropagation();
      const edge = (btn.dataset.edge || "top") as EdgePosition;
      await setEdgeAndMonitor(edge, currentConfig.monitor_index);
    });
  });

  // Ensure window is placed properly on initial boot
  try {
    await invoke("sync_overlay_position");
  } catch {
    // Graceful fallback
  }
}

window.addEventListener("DOMContentLoaded", init);
