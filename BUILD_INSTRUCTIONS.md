# 🚀 Hướng dẫn Build Cache Manager cho Windows

## 📋 Yêu cầu hệ thống

1. **Rust Toolchain**: Cài đặt Rust từ https://rustup.rs/
2. **Windows SDK**: Đảm bảo có Visual Studio Build Tools hoặc Visual Studio

## 📁 Cấu trúc thư mục (CẦN THIẾT)

```
cache_manager/
├── Cargo.toml          # BẮT BUỘC - File cấu hình Cargo
├── .gitignore          # Tùy chọn - Git ignore
└── src/
    └── main.rs         # BẮT BUỘC - Code chính
```

**LƯU Ý**: KHÔNG cần file `build.rs`

## ⚙️ Các bước build (CHI TIẾT)

### Bước 1: Tạo project
```bash
cargo new cache_manager
cd cache_manager
```

### Bước 2: Thay thế nội dung các file

#### File `Cargo.toml` (thay thế toàn bộ):
```toml
[package]
name = "cache_manager"
version = "1.0.0"
edition = "2021"

[dependencies]
eframe = "0.24"
egui = "0.24"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dirs = "5.0"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
panic = "abort"

[[bin]]
name = "cache_manager"
path = "src/main.rs"
```

#### File `src/main.rs` (copy từ artifact)
- Copy toàn bộ code từ artifact "Cache Manager Application"

### Bước 3: XÓA file build.rs (nếu có)
```bash
# Xóa file build.rs
del build.rs       # Windows CMD
# hoặc
rm build.rs        # PowerShell/Git Bash
```

### Bước 4: BUILD FILE .EXE
```bash
# Xóa cache cũ
cargo clean

# Build release - Lệnh này TẠO file .exe
cargo build --release
```

### Bước 5: Tìm file .exe
File `.exe` sẽ được tạo tại:
```
cache_manager/target/release/cache_manager.exe
```

Đường dẫn đầy đủ từ thư mục project:
```
.\target\release\cache_manager.exe
```

## 🔍 Kiểm tra file .exe đã được tạo

```bash
# Kiểm tra file có tồn tại
dir target\release\cache_manager.exe

# Xem kích thước
dir target\release\*.exe

# Chạy thử
target\release\cache_manager.exe
```

## ⚠️ Khắc phục lỗi thường gặp

### 1. Lỗi: "could not compile..."
**Nguyên nhân**: Code có lỗi syntax hoặc thiếu dependencies

**Giải pháp**:
```bash
# Kiểm tra lỗi chi tiết
cargo build --release --verbose

# Đảm bảo đã copy đúng code từ artifact
```

### 2. Không tìm thấy file .exe
**Nguyên nhân**: Đang tìm ở sai thư mục

**Giải pháp**:
```bash
# Liệt kê tất cả file .exe trong project
dir /s *.exe

# File .exe CHỈ có ở: target\release\cache_manager.exe
```

### 3. Lỗi: "linker `link.exe` not found"
**Nguyên nhân**: Chưa cài Visual Studio Build Tools

**Giải pháp**:
1. Download: https://visualstudio.microsoft.com/downloads/
2. Cài đặt "Desktop development with C++"
3. Restart terminal
4. Chạy lại `cargo build --release`

### 4. Build thành công nhưng vẫn thấy console window
**Nguyên nhân**: Đang chạy debug build

**Giải pháp**:
```bash
# Phải dùng --release để ẩn console
cargo build --release

# KHÔNG dùng (sẽ có console):
cargo build
cargo run
```

## ✅ Checklist build thành công

- [ ] File `Cargo.toml` đúng nội dung
- [ ] File `src/main.rs` có dòng đầu: `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`
- [ ] KHÔNG có file `build.rs`
- [ ] Chạy `cargo clean`
- [ ] Chạy `cargo build --release`
- [ ] File `target\release\cache_manager.exe` tồn tại
- [ ] Chạy file .exe không có console window

## 🎯 Các lệnh build hữu ích

```bash
# Build release (TẠO file .exe)
cargo build --release

# Build và chạy luôn
cargo run --release

# Kiểm tra code (không tạo .exe)
cargo check

# Xóa cache build
cargo clean

# Build với thông tin chi tiết
cargo build --release --verbose

# Xem kích thước
dir target\release\cache_manager.exe
```

## 📊 Kích thước file .exe

- **Thường**: 5-10 MB
- **Sau khi strip**: 3-5 MB  
- **Với UPX nén**: 2-4 MB

## 🚀 Phân phối file .exe

File `cache_manager.exe` có thể:
- ✅ Chạy độc lập, không cần cài Rust
- ✅ Copy sang máy khác và chạy ngay
- ✅ Không cần file Cargo.toml hay src/

Chỉ cần copy file:
```
target\release\cache_manager.exe
```

---

**Nếu vẫn gặp lỗi, hãy gửi thông báo lỗi để tôi giúp bạn! 🎉**