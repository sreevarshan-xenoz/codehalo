# CodeHalo

> **Your AI coding agents, at a glance.**

CodeHalo is a native, hardware-accelerated desktop overlay for **Windows and Linux** that monitors your AI coding agent usage (Claude Code, OpenAI Codex, Cursor, Gemini/Antigravity, and more) in one unified, unobtrusive screen-edge HUD.

---

## 🔒 Tech Stack

- **Core Engine**: Rust (Tokio, Rusqlite, Serde, Reqwest)
- **UI Framework**: Qt 6 Quick / QML (Direct GPU rendering, fluid 60/120fps animations)
- **Rust ↔ Qt Bridge**: CXX-Qt
- **Build System**: CMake + Cargo (standalone native binary)
- **Target OS**: Windows 10/11 & Linux (Wayland-first with X11 fallback)

---

## 🏗️ Architecture

CodeHalo follows a strict architectural contract:
> **Rust owns the application. QML owns the presentation.**

```text
Rust Core ──▶ CXX-Qt Bridge ──▶ Qt Quick / QML Presentation
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for full system specifications.

---

## 📦 Building from Source

### Prerequisites
- **Rust**: 1.75+
- **CMake**: 3.22+
- **Qt 6**: 6.5+ (Quick, QML, QuickControls2)
- **C++ Compiler**: MSVC 2019+ or GCC/Clang with C++20 support

### Build Commands
```bash
# Configure with CMake
cmake -B build -S .

# Build binary
cmake --build build --config Release
```

---

## 📄 License

MIT
