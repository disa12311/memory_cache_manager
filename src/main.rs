// Cargo.toml dependencies cần thiết:
// [package]
// name = "cache_manager"
// version = "1.0.0"
// edition = "2021"
//
// [dependencies]
// eframe = "0.24"
// egui = "0.24"
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// dirs = "5.0"
//
// [profile.release]
// opt-level = 3
// lto = true
// codegen-units = 1
// strip = true

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    cache_threshold_gb: f32,
    auto_clean_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cache_threshold_gb: 10.0,
            auto_clean_enabled: true,
        }
    }
}

struct CacheManager {
    config: Config,
    last_clean_time: Arc<Mutex<Option<Instant>>>,
    cache_size_gb: f32,
    is_cleaning: bool,
    status_message: String,
}

impl CacheManager {
    fn new() -> Self {
        let config = Self::load_config().unwrap_or_default();
        
        Self {
            config,
            last_clean_time: Arc::new(Mutex::new(None)),
            cache_size_gb: 0.0,
            is_cleaning: false,
            status_message: String::from("Sẵn sàng"),
        }
    }

    fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
        let config_path = Self::config_path();
        let data = std::fs::read_to_string(config_path)?;
        Ok(serde_json::from_str(&data)?)
    }

    fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path();
        let data = serde_json::to_string_pretty(&self.config)?;
        std::fs::write(config_path, data)?;
        Ok(())
    }

    fn config_path() -> PathBuf {
        // Sử dụng thư mục AppData cho Windows
        let mut path = if cfg!(target_os = "windows") {
            // Windows: C:\Users\<username>\AppData\Roaming\CacheManager
            dirs::config_dir()
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
        } else {
            std::env::current_dir().unwrap_or_default()
        };
        
        // Tạo thư mục nếu chưa tồn tại
        if cfg!(target_os = "windows") {
            path.push("CacheManager");
            std::fs::create_dir_all(&path).ok();
        }
        
        path.push("cache_manager_config.json");
        path
    }

    fn get_cache_size(&mut self) -> f32 {
        // Windows cache directories
        let temp_dirs = if cfg!(target_os = "windows") {
            vec![
                std::env::temp_dir(), // C:\Users\<user>\AppData\Local\Temp
                PathBuf::from(std::env::var("LOCALAPPDATA").unwrap_or_default())
                    .join("Temp"),
            ]
        } else {
            vec![std::env::temp_dir()]
        };

        let mut total_size: u64 = 0;
        for dir in temp_dirs {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    if let Ok(metadata) = entry.metadata() {
                        total_size += metadata.len();
                    }
                }
            }
        }

        (total_size as f32) / (1024.0 * 1024.0 * 1024.0) // Convert to GB
    }

    fn clean_cache(&mut self) {
        self.is_cleaning = true;
        self.status_message = String::from("Đang xóa cache...");

        // Windows temp directories
        let temp_dirs = if cfg!(target_os = "windows") {
            vec![
                std::env::temp_dir(),
                PathBuf::from(std::env::var("LOCALAPPDATA").unwrap_or_default())
                    .join("Temp"),
            ]
        } else {
            vec![std::env::temp_dir()]
        };

        let mut cleaned_count = 0;
        let mut cleaned_size: u64 = 0;

        for temp_dir in temp_dirs {
            if let Ok(entries) = std::fs::read_dir(&temp_dir) {
                for entry in entries.flatten() {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            let file_size = metadata.len();
                            // Chỉ xóa file nếu có thể (bỏ qua file đang được sử dụng)
                            if std::fs::remove_file(entry.path()).is_ok() {
                                cleaned_count += 1;
                                cleaned_size += file_size;
                            }
                        }
                    }
                }
            }
        }

        let cleaned_gb = (cleaned_size as f32) / (1024.0 * 1024.0 * 1024.0);
        self.status_message = format!("✅ Đã xóa {} file ({:.2} GB)", cleaned_count, cleaned_gb);

        *self.last_clean_time.lock().unwrap() = Some(Instant::now());
        self.is_cleaning = false;
        self.cache_size_gb = self.get_cache_size();
    }

    fn should_auto_clean(&self) -> bool {
        if !self.config.auto_clean_enabled {
            return false;
        }

        // Kiểm tra nếu đã qua 30 giây từ lần xóa cuối
        if let Some(last_time) = *self.last_clean_time.lock().unwrap() {
            if last_time.elapsed() < Duration::from_secs(30) {
                return false;
            }
        }

        // Kiểm tra nếu cache vượt ngưỡng
        self.cache_size_gb >= self.config.cache_threshold_gb
    }
}

impl eframe::App for CacheManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update cache size
        self.cache_size_gb = self.get_cache_size();

        // Auto clean nếu cần
        if self.should_auto_clean() && !self.is_cleaning {
            self.clean_cache();
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(20, 20, 20)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    
                    ui.heading(
                        egui::RichText::new("🗂️ Cache Manager")
                            .size(28.0)
                            .color(egui::Color32::from_rgb(100, 200, 255))
                    );
                    
                    ui.add_space(30.0);

                    // Hiển thị kích thước cache hiện tại
                    ui.label(
                        egui::RichText::new(format!("📊 Kích thước cache hiện tại: {:.2} GB", self.cache_size_gb))
                            .size(18.0)
                            .color(egui::Color32::WHITE)
                    );

                    ui.add_space(10.0);

                    // Status message
                    ui.label(
                        egui::RichText::new(&self.status_message)
                            .size(14.0)
                            .color(egui::Color32::from_rgb(150, 150, 150))
                    );

                    ui.add_space(30.0);

                    // Threshold slider
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("🎚️ Ngưỡng bộ nhớ đệm (GB):")
                                .size(16.0)
                                .color(egui::Color32::WHITE)
                        );
                    });

                    ui.add_space(10.0);

                    let mut threshold = self.config.cache_threshold_gb;
                    ui.add(
                        egui::Slider::new(&mut threshold, 1.0..=100.0)
                            .text("GB")
                            .step_by(1.0)
                    );
                    self.config.cache_threshold_gb = threshold;

                    ui.add_space(10.0);
                    
                    ui.label(
                        egui::RichText::new(format!("Sẽ tự động xóa khi cache đạt {:.0} GB", threshold))
                            .size(13.0)
                            .color(egui::Color32::from_rgb(180, 180, 180))
                    );

                    ui.add_space(20.0);

                    // Auto clean checkbox
                    ui.checkbox(
                        &mut self.config.auto_clean_enabled,
                        egui::RichText::new("🔄 Bật tự động xóa sau 30 giây")
                            .size(15.0)
                            .color(egui::Color32::WHITE)
                    );

                    ui.add_space(30.0);

                    // Save button
                    if ui.add_sized(
                        [200.0, 40.0],
                        egui::Button::new(
                            egui::RichText::new("💾 Lưu cấu hình")
                                .size(16.0)
                        )
                    ).clicked() {
                        match self.save_config() {
                            Ok(_) => self.status_message = String::from("✅ Đã lưu cấu hình"),
                            Err(e) => self.status_message = format!("❌ Lỗi: {}", e),
                        }
                    }

                    ui.add_space(10.0);

                    // Manual clean button
                    if ui.add_sized(
                        [200.0, 40.0],
                        egui::Button::new(
                            egui::RichText::new("🧹 Xóa cache ngay")
                                .size(16.0)
                        )
                    ).clicked() {
                        self.clean_cache();
                    }

                    ui.add_space(20.0);

                    // Progress indicator
                    if self.is_cleaning {
                        ui.spinner();
                    }

                    ui.add_space(20.0);

                    // Info về thời gian xóa cuối
                    if let Some(last_time) = *self.last_clean_time.lock().unwrap() {
                        let elapsed = last_time.elapsed().as_secs();
                        ui.label(
                            egui::RichText::new(format!("⏱️ Lần xóa cuối: {} giây trước", elapsed))
                                .size(13.0)
                                .color(egui::Color32::from_rgb(150, 150, 150))
                        );
                    }
                });
            });

        // Request repaint để cập nhật UI
        ctx.request_repaint_after(Duration::from_secs(1));
    }
}

fn main() -> Result<(), eframe::Error> {
    // Ẩn console window trên Windows khi build release
    #[cfg(all(target_os = "windows", not(debug_assertions)))]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        // This will be handled by Windows subsystem setting
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 550.0])
            .with_min_inner_size([400.0, 450.0])
            .with_resizable(true)
            .with_title("Cache Manager"),
        ..Default::default()
    };

    eframe::run_native(
        "Cache Manager",
        options,
        Box::new(|_cc| Box::new(CacheManager::new())),
    )
}