# 🚀 Hướng dẫn Build Cache Manager cho Windows

## 📋 Yêu cầu hệ thống

1. **Rust Toolchain**: Cài đặt Rust từ https://rustup.rs/
2. **Windows SDK**: Đảm bảo có Visual Studio Build Tools hoặc Visual Studio

## 📁 Cấu trúc thư mục

```
cache_manager/
├── Cargo.toml          # File cấu hình Cargo
├── build.rs            # (Tùy chọn - có thể xóa)
├── .gitignore          # Git ignore file
└── src/
    └── main.rs         # Code chính
```

## ⚙️ Các bước build

### 1. Tạo project mới
```bash
cargo new cache_manager
cd cache_manager
```

### 2. Copy các file
- Copy nội dung `Cargo.toml` vào file `Cargo.toml`
- Copy nội dung code chính vào `src/main.rs`
- Copy nội dung `.gitignore` vào file `.gitignore`
- File `build.rs` là tùy chọn, có thể bỏ qua

### 3. Build release
```bash
# Build phiên bản release (tối ưu hóa)
cargo build --release

# File .exe sẽ được tạo tại:
# target/release/cache_manager.exe
```

### 4. Chạy thử
```bash
# Chạy file .exe
target\release\cache_manager.exe

# Hoặc chạy trực tiếp bằng cargo
cargo run --release
```

## 🎯 Ẩn Console Window

Console window đã được ẩn tự động trong release build nhờ dòng này trong `src/main.rs`:
```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

- ✅ **Release build**: Không có console window
- ✅ **Debug build**: Có console window (để debug)

## 📦 File cấu hình

Ứng dụng sẽ lưu cấu hình tại:
```
C:\Users\<username>\AppData\Roaming\CacheManager\cache_manager_config.json
```

## 🔧 Khắc phục sự cố

### Lỗi: "linker `link.exe` not found"
**Giải pháp**: Cài đặt Visual Studio Build Tools
```bash
# Download và cài đặt từ:
# https://visualstudio.microsoft.com/downloads/
# Chọn "Desktop development with C++"
```

### Lỗi compilation
**Giải pháp**: Cập nhật Rust toolchain
```bash
rustup update stable
```

### File .exe quá lớn
**Giải pháp**: 
1. Đảm bảo build với `--release`
2. Sử dụng UPX để nén (có thể giảm 50-70% kích thước)
3. Kiểm tra `Cargo.toml` đã có cấu hình `[profile.release]`

### Vẫn thấy console window
**Giải pháp**:
1. Đảm bảo build với `--release` (không phải `--debug`)
2. Kiểm tra dòng đầu tiên trong `src/main.rs` có:
   ```rust
   #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
   ```

## 🎯 Tùy chọn build nâng cao

### Build với UPX compression (giảm kích thước)
```bash
# Cài đặt UPX
# Download từ: https://upx.github.io/

# Build
cargo build --release

# Nén file .exe
upx --best --lzma target\release\cache_manager.exe
```

### Build cho nhiều target
```bash
# Build cho Windows 64-bit
cargo build --release --target x86_64-pc-windows-msvc

# Build cho Windows 32-bit
rustup target add i686-pc-windows-msvc
cargo build --release --target i686-pc-windows-msvc
```

## ✅ Kiểm tra sau build

1. **Chạy thử file .exe**:
   ```bash
   target\release\cache_manager.exe
   ```

2. **Kiểm tra không có console window xuất hiện** (chỉ trong release build)

3. **Kiểm tra file config được tạo tại AppData\Roaming\CacheManager**

4. **Test các tính năng**:
   - Điều chỉnh threshold slider
   - Lưu cấu hình
   - Xóa cache thủ công
   - Để chạy nền và kiểm tra auto-clean

## 🚀 Phân phối

File `.exe` có thể chạy độc lập, không cần cài đặt Rust. Copy file `target\release\cache_manager.exe` và chia sẻ!

## 📊 Kích thước file dự kiến

- **Release build**: ~5-10 MB  
- **Với UPX compression**: ~2-4 MB

## 🎨 Các lệnh build hữu ích

```bash
# Build release
cargo build --release

# Build và chạy release
cargo run --release

# Xóa cache build
cargo clean

# Kiểm tra code không build
cargo check

# Build với output verbose
cargo build --release --verbose

# Xem kích thước các dependencies
cargo bloat --release
```

---

**Chúc bạn build thành công! 🎉**