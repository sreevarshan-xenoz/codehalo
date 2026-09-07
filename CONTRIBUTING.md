# Contributing to CodeHalo

Thank you for your interest in contributing to **CodeHalo**! We are building the cleanest, lightest cross-platform HUD for AI coding tools.

---

## Code of Conduct

Please be polite, constructive, and respectful to fellow contributors and users.

---

## Development Setup

1. **Clone the repository**:
   ```bash
   git clone https://github.com/sreevarshan-xenoz/agent-hud.git
   cd agent-hud
   ```

2. **Install frontend dependencies**:
   ```bash
   npm install
   ```

3. **Prerequisites**:
   - Rust 1.75+ (via `rustup`)
   - Node.js 18+
   - Platform build tools:
     - **Windows**: Visual Studio C++ Build Tools & WebView2
     - **Linux**: `libwebkit2gtk-4.1-dev`, `build-essential`, `libssl-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`

4. **Run locally**:
   ```bash
   npm run tauri dev
   ```

---

## Engineering Rules

1. **Keep it lightweight**: Low CPU/memory usage is a core feature. Never compromise performance for excessive styling or unnecessary background polling.
2. **Local-first & Privacy**: No telemetry, no central analytics server, and never transmit user credentials.
3. **Transparent Usage Data**: Clearly distinguish between *Official*, *Local/Derived*, and *Estimated* quotas. Never present an estimate as an official provider value.
4. **Cross-Platform Abstraction**: Isolate Windows and Linux (Wayland/X11) platform code behind clean traits/modules.

---

## Adding a New Provider

1. Implement the `UsageProvider` trait in `src-tauri/src/provider.rs`.
2. Provide independent unit tests for parsing usage quotas/tokens.
3. Submit a Pull Request with a short description and test instructions.
