# CodeHalo Architecture Specification

> **Native Desktop Shell. Rust Brain. QML Face. GPU-Smooth HUD. Local-First.**

CodeHalo is a high-performance, lightweight, cross-platform (Windows & Linux) desktop overlay for monitoring AI coding agent usage (e.g., Claude Code, OpenAI Codex, Cursor, Gemini/Antigravity).

It replaces web-view wrappers with a native **Qt 6 Quick / QML** presentation layer powered by a **Rust** core engine via **CXX-Qt**.

---

## 1. Technology Stack

| Layer | Technology | Purpose |
|---|---|---|
| **Core Engine** | **Rust** | Providers, networking, storage, state management, platform abstraction |
| **UI Framework** | **Qt 6 (Qt Quick / QML)** | Hardware-accelerated native HUD, fluid 60/120fps animations, glass/blur effects |
| **Rust ↔ Qt Bridge** | **CXX-Qt** | Direct FFI bindings between Rust QObjects and QML declarative bindings |
| **Build System** | **CMake + Cargo** | Native build pipeline producing a standalone executable (`codehalo.exe`) |
| **Database** | **SQLite (rusqlite)** | Local-only settings, token cache, and session metrics history |
| **Async Runtime** | **Tokio** | Asynchronous background polling and provider rate limit backoff |
| **Networking** | **Reqwest** | Direct provider API requests |

---

## 2. Layered Architecture

```text
┌────────────────────────────────────────────────────────┐
│                   QML Presentation Layer               │
│               Qt Quick 6 / Hardware-Accelerated        │
│  - Collapsed HUD pill & animated halo ring             │
│  - Expanded drawer with fluid spring/cubic transitions │
│  - Theme singletons (Colors.qml, Typography.qml)       │
├────────────────────────────────────────────────────────┤
│                   CXX-Qt Bridge (FFI)                  │
│  - QObject property bindings (expanded, edge, scale)   │
│  - Invokable Rust signals & slots                      │
├────────────────────────────────────────────────────────┤
│                      Rust Core                         │
│  - Application state & event loop                      │
│  - Provider registry, rate limit scheduler             │
│  - SQLite storage (local settings & usage logs)        │
├────────────────────────────────────────────────────────┤
│                 Platform Abstraction                   │
│  - Window edge positioning, DPI scaling, multi-monitor │
│  - OS keyring security (Windows DPAPI, Secret Service) │
├──────────────────────────┬─────────────────────────────┤
│         Windows          │            Linux            │
│   Win32 / DWM / DPAPI    │   Wayland Layer Shell / X11 │
└──────────────────────────┴─────────────────────────────┘
```

---

## 3. Directory Layout

```text
codehalo/
├── CMakeLists.txt              # Root build orchestrator (Qt 6 + CXX-Qt)
├── Cargo.toml                  # Cargo workspace root
├── rust/
│   ├── Cargo.toml              # Rust core dependencies (cxx-qt, tokio, rusqlite)
│   ├── build.rs                # CXX-Qt build script
│   └── src/
│       ├── main.cpp            # Qt application bootstrapper
│       ├── lib.rs              # Module declarations
│       ├── core/               # State, models, event loop
│       ├── platform/           # Multi-monitor & DPI OS abstractions
│       └── qt/                 # CXX-Qt bridge (Rust QObjects)
├── qml/
│   ├── Main.qml                # Primary transparent always-on-top window
│   ├── components/             # Halo.qml, Hud.qml
│   └── theme/                  # Colors.qml, Typography.qml
└── assets/
    └── icons/                  # codehalo.png logo and app icons
```

---

## 4. Privacy & Local-First Guarantees
- **No telemetry & no external servers**: All requests are made directly to provider endpoints or read from local agent sessions.
- **Native OS credential storage**: Sensitive keys are encrypted with Windows DPAPI or Linux Secret Service.
