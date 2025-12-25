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
- Windows OS (for building on Linux/Codespaces, use cross-compilation)

### Build from source (trên Codespaces/Linux)

```bash
# Đã có config trong .cargo/config.toml
# Build release cho Windows
cargo build --release

# File .exe sẽ ở:
# target/x86_64-pc-windows-gnu/release/memory-cache-manager.exe
```

### Build trên Windows

```bash
# Build release
cargo build --release

# File .exe tại:
# target/release/memory-cache-manager.exe
```

## 📁 Project Structure

```
memory-cache-manager/
├── .cargo/
│   └── config.toml      # Cross-compile config
├── Cargo.toml           # Rust dependencies
├── build.rs             # Tauri build script
├── tauri.conf.json      # Tauri configuration
├── src/
│   ├── main.rs          # Rust backend (Windows API)
│   └── lib.rs           # Library entry
└── ui/
    └── index.html       # Frontend UI
```

## 🔧 Setup ngay

### Bước 1: Tạo cấu trúc thư mục
```bash
mkdir -p src ui .cargo
```

### Bước 2: Copy các file
- `Cargo.toml` (từ artifact 1)
- `build.rs` (từ artifact 4)
- `tauri.conf.json` (từ artifact mới vừa update)
- `.cargo/config.toml` (từ document 2)
- `src/main.rs` (từ artifact 2)
- `src/lib.rs` (từ artifact 3)
- `ui/index.html` (từ artifact 6)

### Bước 3: Build
```bash
cargo build --release
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
# Run in dev mode (trên Windows)
cargo run --release

# Build release
cargo build --release

# Clean build artifacts
cargo clean
```

## 🐛 Troubleshooting

### Lỗi `tauri.conf.json`
- Đảm bảo file có đúng format (đã update trong artifact)
- File phải có `identifier` trong `bundle`

### Build failed
```bash
# Xóa cache và build lại
cargo clean
cargo build --release
```

### Cross-compile issues
- Đảm bảo đã cài `mingw-w64`
- File `.cargo/config.toml` phải có trong project root

## 📝 License

MIT License - Feel free to use and modify

---

**Made with ❤️ using Rust + Tauri**