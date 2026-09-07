# CodeHalo

> **Your AI coding agents, at a glance.**

CodeHalo is a modern, lightweight, transparent desktop overlay for **Windows and Linux** that monitors your AI coding agent usage (Claude Code, OpenAI Codex, Cursor, Gemini/Antigravity, and more) in one unified screen-edge HUD.

---

## Features

- 🪟 **Minimal Screen-Edge Overlay**: Collapsed pill HUD that expands on hover/click into detailed breakdown metrics.
- ⚡ **Lightweight & Native**: Built with Tauri 2 + Rust for minimal memory and CPU footprint.
- 🔒 **Privacy-First & Local**: Direct provider polling with local credential management (Windows Credential Manager / Linux Secret Service). No intermediary servers or telemetry.
- 🖥️ **Multi-Monitor Aware**: Position on Top, Bottom, Left, or Right edges on any connected monitor with proper DPI scaling.
- 🔌 **Extensible Provider System**: Pluggable architecture distinguishing official, local, and estimated metrics.

---

## Development

### Prerequisites

- **Node.js**: v18+
- **Rust**: 1.75+
- **Platform Dependencies**:
  - Windows: Visual Studio C++ Build Tools & WebView2
  - Linux: `libwebkit2gtk-4.1-dev`, `build-essential`, `libssl-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`

### Getting Started

```bash
# Install frontend dependencies
npm install

# Run desktop app in development mode
npm run tauri dev
```

---

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed structural designs, platform abstractions, and provider contracts.

---

## License

MIT
