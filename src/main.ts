import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import type { ProviderUsage } from "./types";

const appWindow = getCurrentWindow();

// Default provider snapshot state
let providers: ProviderUsage[] = [
  {
    id: "claude",
    name: "Claude Code",
    percentage: 82,
    usedDisplay: "820k / 1M tokens",
    limitDisplay: "Daily limit",
    status: "ok",
    isEstimate: false,
    lastUpdated: "12s ago",
  },
  {
    id: "codex",
    name: "OpenAI Codex",
    percentage: 61,
    usedDisplay: "61 / 100 requests",
    limitDisplay: "5-hour window",
    status: "ok",
    isEstimate: false,
    lastUpdated: "45s ago",
  },
  {
    id: "cursor",
    name: "Cursor Pro",
    percentage: 48,
    usedDisplay: "240 / 500 fast calls",
    limitDisplay: "Monthly reset in 12d",
    status: "ok",
    isEstimate: false,
    lastUpdated: "2m ago",
  },
];

let isExpanded = false;

function renderHud() {
  const container = document.getElementById("provider-container");
  if (!container) return;

  // Update collapsed pill summary
  const pillClaude = document.getElementById("pill-claude");
  const pillCodex = document.getElementById("pill-codex");
  if (pillClaude && providers[0]) pillClaude.textContent = `${providers[0].percentage}%`;
  if (pillCodex && providers[1]) pillCodex.textContent = `${providers[1].percentage}%`;

  // Render expanded items
  container.innerHTML = providers
    .map((p) => {
      const statusClass =
        p.percentage > 85 ? "status-critical" : p.percentage > 70 ? "status-warning" : "status-ok";
      return `
        <div class="provider-item" data-id="${p.id}">
          <div class="provider-item-header">
            <span class="provider-name">
              ${p.name}
              ${p.isEstimate ? '<span class="provider-badge">Estimate</span>' : ""}
            </span>
            <span class="provider-stat">${p.percentage}%</span>
          </div>
          <div class="progress-track">
            <div class="progress-fill ${statusClass}" style="width: ${p.percentage}%"></div>
          </div>
          <div style="display: flex; justify-content: space-between; font-size: 10px; color: var(--text-muted); margin-top: 2px;">
            <span>${p.usedDisplay}</span>
            <span>${p.limitDisplay}</span>
          </div>
        </div>
      `;
    })
    .join("");
}

async function setExpanded(expanded: boolean) {
  isExpanded = expanded;
  const root = document.getElementById("hud-root");
  if (!root) return;

  if (isExpanded) {
    root.classList.add("expanded");
    // Expand Tauri window height
    await appWindow.setSize(new LogicalSize(380, 290));
  } else {
    root.classList.remove("expanded");
    // Shrink Tauri window to compact pill
    await appWindow.setSize(new LogicalSize(380, 48));
  }
}

async function init() {
  renderHud();

  const pill = document.getElementById("hud-pill");
  const btnCollapse = document.getElementById("btn-collapse");
  const btnRefresh = document.getElementById("btn-refresh");

  pill?.addEventListener("click", () => {
    setExpanded(!isExpanded);
  });

  btnCollapse?.addEventListener("click", (e) => {
    e.stopPropagation();
    setExpanded(false);
  });

  btnRefresh?.addEventListener("click", async (e) => {
    e.stopPropagation();
    try {
      const res = await invoke<ProviderUsage[]>("get_provider_usages");
      if (res && res.length > 0) {
        providers = res;
        renderHud();
      }
    } catch {
      // Keep UI functional during backend initialization
      renderHud();
    }
  });

  // Query rust backend for initial monitor / positioning sync
  try {
    await invoke("sync_overlay_position");
  } catch {
    // Graceful fallback
  }
}

window.addEventListener("DOMContentLoaded", init);
