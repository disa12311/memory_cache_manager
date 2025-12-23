# 🧠 Memory Cache Manager v1.0

Advanced Memory Cache Cleaner for Windows with Tauri UI

## ✨ Features

- **Real Memory Cache Cleaning**: Uses Windows API to actually clear memory cache
- **Modern Tauri UI**: Beautiful, responsive interface
- **Dual Threshold System**: Start and stop thresholds for smart cleaning
- **Auto-Clean**: Automatic cleaning every 30 seconds when threshold is reached
- **Real-time Monitoring**: Live memory usage display
- **Lightweight**: Small binary size with native performance

## 🚀 Installation

### Prerequisites
- Rust 1.70+
- Node.js (not required, UI is pure HTML/JS)
- Windows OS

### Build from source

```bash
# Install Tauri CLI
cargo install tauri-cli

# Build release
cargo tauri build

# The .exe will be in:
# target/release/memory-cache-manager.exe
```

## 📁 Project Structure

```
memory-cache-manager/
├── Cargo.toml          # Rust dependencies
├── build.rs            # Tauri build script
├── tauri.conf.json     # Tauri configuration
├── src/
│   ├── main.rs         # Rust backend (Windows API)
│   └── lib.rs          # Library entry
└── ui/
    └── index.html      # Frontend UI
```

## 🎯 How It Works

### Backend (Rust + Windows API)

1. **Get Memory Info**: Uses `GlobalMemoryStatusEx` to get real-time memory stats
2. **Clean Cache**: 
   - Allocates and frees memory chunks to force Windows to page out cached data
   - Calls `EmptyWorkingSet` to trim working set
   - Actually reduces cached memory, not just estimates

### Frontend (HTML/JS)

- Modern gradient UI with smooth animations
- Real-time progress bar and statistics
- Interactive sliders for threshold configuration
- Auto-clean with 30-second cooldown

## 🔧 Configuration

- **Start Threshold**: Memory usage to trigger cleaning (512-8192 MB)
- **Stop Threshold**: Target memory after cleaning (256-4096 MB)
- **Auto-Clean**: Enable/disable automatic cleaning

## ⚠️ Notes

- **Run as Administrator** for best results
- Windows-only (uses WinAPI)
- Cleaning process takes 2-10 seconds depending on target
- Safe: Only clears cache, doesn't touch system or application data

## 📊 Comparison

| Feature | v1.0 (Tauri) | Previous (eframe) |
|---------|--------------|-------------------|
| UI Framework | Tauri + HTML | eframe/egui |
| Memory Cleaning | Real (WinAPI) | Estimated (PowerShell) |
| Size | ~8-12 MB | ~5-10 MB |
| Performance | Fast | Laggy with PS commands |
| Cross-platform UI | Easy to update | Rust only |

## 🛠️ Development

```bash
# Run in dev mode
cargo tauri dev

# Build release
cargo tauri build

# Clean build artifacts
cargo clean
```

## 📝 License

MIT License - Feel free to use and modify

## 🤝 Contributing

Contributions welcome! Please feel free to submit a Pull Request.

---

**Made with ❤️ using Rust + Tauri**