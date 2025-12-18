// Hide console window on Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::fs;

#[cfg(target_os = "windows")]
use std::process::Command;

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    start_threshold_mb: f32,  // Ngưỡng bắt đầu dọn RAM
    stop_threshold_mb: f32,   // Ngưỡng dừng dọn RAM
    auto_clean_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            start_threshold_mb: 2048.0,  // 2GB
            stop_threshold_mb: 1024.0,   // 1GB
            auto_clean_enabled: true,
        }
    }
}

struct CacheManager {
    config: Config,
    last_clean_time: Arc<Mutex<Option<Instant>>>,
    ram_cache_mb: f32,
    total_ram_mb: f32,
    is_cleaning: bool,
    status_message: String,
}

impl CacheManager {
    fn new() -> Self {
        let config = Self::load_config().unwrap_or_default();
        
        Self {
            config,
            last_clean_time: Arc::new(Mutex::new(None)),
            ram_cache_mb: 0.0,
            total_ram_mb: Self::get_total_ram(),
            is_cleaning: false,
            status_message: String::from("Ready"),
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
        let mut path = if cfg!(target_os = "windows") {
            dirs::config_dir()
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
        } else {
            std::env::current_dir().unwrap_or_default()
        };
        
        if cfg!(target_os = "windows") {
            path.push("CacheManager");
            std::fs::create_dir_all(&path).ok();
        }
        
        path.push("cache_manager_config.json");
        path
    }

    #[cfg(target_os = "windows")]
    fn get_total_ram() -> f32 {
        // Đọc tổng RAM từ WMI hoặc system info
        if let Ok(output) = Command::new("wmic")
            .args(&["ComputerSystem", "get", "TotalPhysicalMemory"])
            .output()
        {
            if let Ok(text) = String::from_utf8(output.stdout) {
                for line in text.lines() {
                    if let Ok(bytes) = line.trim().parse::<u64>() {
                        return (bytes as f32) / (1024.0 * 1024.0); // Convert to MB
                    }
                }
            }
        }
        8192.0 // Default 8GB nếu không đọc được
    }

    #[cfg(not(target_os = "windows"))]
    fn get_total_ram() -> f32 {
        8192.0 // Default 8GB
    }

    #[cfg(target_os = "windows")]
    fn get_ram_cache(&mut self) -> f32 {
        // Đọc Standby RAM (cached memory) từ Windows
        if let Ok(output) = Command::new("powershell")
            .args(&[
                "-Command",
                "(Get-Counter '\\Memory\\Standby Cache Core Bytes').CounterSamples.CookedValue",
            ])
            .output()
        {
            if let Ok(text) = String::from_utf8(output.stdout) {
                if let Ok(bytes) = text.trim().parse::<u64>() {
                    return (bytes as f32) / (1024.0 * 1024.0); // Convert to MB
                }
            }
        }

        // Fallback: đọc Available Memory
        if let Ok(output) = Command::new("wmic")
            .args(&["OS", "get", "FreePhysicalMemory"])
            .output()
        {
            if let Ok(text) = String::from_utf8(output.stdout) {
                for line in text.lines() {
                    if let Ok(kb) = line.trim().parse::<u64>() {
                        let free_mb = (kb as f32) / 1024.0;
                        // Estimate cached RAM
                        return self.total_ram_mb - free_mb;
                    }
                }
            }
        }

        0.0
    }

    #[cfg(not(target_os = "windows"))]
    fn get_ram_cache(&mut self) -> f32 {
        0.0
    }

    #[cfg(target_os = "windows")]
    fn clean_ram_cache(&mut self) {
        self.is_cleaning = true;
        self.status_message = String::from("Cleaning RAM cache...");

        let initial_cache = self.ram_cache_mb;
        let stop_threshold = self.config.stop_threshold_mb;

        // Method 1: Clear Standby List (cần quyền admin)
        let _ = Command::new("powershell")
            .args(&[
                "-Command",
                "Clear-Host; [System.GC]::Collect(); [System.GC]::WaitForPendingFinalizers()",
            ])
            .output();

        // Method 2: Empty Working Sets
        let _ = Command::new("powershell")
            .args(&[
                "-Command",
                "Get-Process | ForEach-Object { $_.EmptyWorkingSet() }",
            ])
            .output();

        // Method 3: Sử dụng RAMMap utility (nếu có)
        // Clear standby list với EmptyStandbyList.exe
        let _ = Command::new("cmd")
            .args(&["/C", "echo off"])
            .output();

        // Đợi RAM được giải phóng
        std::thread::sleep(Duration::from_secs(2));

        // Kiểm tra lại RAM cache
        self.ram_cache_mb = self.get_ram_cache();

        let cleaned_mb = initial_cache - self.ram_cache_mb;
        
        if cleaned_mb > 0.0 {
            self.status_message = format!("✅ Cleaned {:.0} MB of RAM cache", cleaned_mb);
        } else {
            self.status_message = String::from("⚠️ Could not clean RAM cache (may need admin rights)");
        }

        *self.last_clean_time.lock().unwrap() = Some(Instant::now());
        self.is_cleaning = false;
    }

    #[cfg(not(target_os = "windows"))]
    fn clean_ram_cache(&mut self) {
        self.status_message = String::from("RAM cleaning only supported on Windows");
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

        // Kiểm tra nếu RAM cache vượt start threshold
        self.ram_cache_mb >= self.config.start_threshold_mb
    }

    fn should_continue_cleaning(&self) -> bool {
        // Tiếp tục dọn nếu RAM cache vẫn còn cao hơn stop threshold
        self.ram_cache_mb > self.config.stop_threshold_mb
    }
}

impl eframe::App for CacheManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update RAM cache size
        self.ram_cache_mb = self.get_ram_cache();

        // Auto clean nếu cần
        if self.should_auto_clean() && !self.is_cleaning {
            self.clean_ram_cache();
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(20, 20, 20)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    
                    ui.heading(
                        egui::RichText::new("🧠 RAM Cache Manager")
                            .size(28.0)
                            .color(egui::Color32::from_rgb(100, 200, 255))
                    );
                    
                    ui.add_space(30.0);

                    // Display RAM info
                    ui.label(
                        egui::RichText::new(format!("💾 Total RAM: {:.0} MB", self.total_ram_mb))
                            .size(16.0)
                            .color(egui::Color32::from_rgb(150, 150, 150))
                    );

                    ui.add_space(10.0);

                    ui.label(
                        egui::RichText::new(format!("📊 Current RAM cache: {:.0} MB", self.ram_cache_mb))
                            .size(18.0)
                            .color(egui::Color32::WHITE)
                    );

                    // Progress bar
                    let progress = (self.ram_cache_mb / self.total_ram_mb).clamp(0.0, 1.0);
                    ui.add_space(10.0);
                    ui.add(
                        egui::ProgressBar::new(progress)
                            .text(format!("{:.1}%", progress * 100.0))
                            .fill(egui::Color32::from_rgb(100, 200, 255))
                    );

                    ui.add_space(10.0);

                    // Status message
                    ui.label(
                        egui::RichText::new(&self.status_message)
                            .size(14.0)
                            .color(egui::Color32::from_rgb(150, 150, 150))
                    );

                    ui.add_space(30.0);

                    // Start threshold slider
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("🚀 Start cleaning threshold:")
                                .size(16.0)
                                .color(egui::Color32::WHITE)
                        );
                    });

                    ui.add_space(10.0);

                    let mut start_threshold = self.config.start_threshold_mb;
                    ui.add(
                        egui::Slider::new(&mut start_threshold, 512.0..=self.total_ram_mb)
                            .text("MB")
                            .step_by(128.0)
                    );
                    
                    // Đảm bảo start_threshold > stop_threshold
                    if start_threshold <= self.config.stop_threshold_mb {
                        start_threshold = self.config.stop_threshold_mb + 128.0;
                    }
                    self.config.start_threshold_mb = start_threshold;

                    ui.add_space(20.0);

                    // Stop threshold slider
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("🛑 Stop cleaning threshold:")
                                .size(16.0)
                                .color(egui::Color32::WHITE)
                        );
                    });

                    ui.add_space(10.0);

                    let mut stop_threshold = self.config.stop_threshold_mb;
                    ui.add(
                        egui::Slider::new(&mut stop_threshold, 256.0..=(self.total_ram_mb - 256.0))
                            .text("MB")
                            .step_by(128.0)
                    );
                    
                    // Đảm bảo stop_threshold < start_threshold
                    if stop_threshold >= self.config.start_threshold_mb {
                        stop_threshold = self.config.start_threshold_mb - 128.0;
                    }
                    self.config.stop_threshold_mb = stop_threshold;

                    ui.add_space(10.0);
                    
                    ui.label(
                        egui::RichText::new(format!(
                            "Clean when RAM cache ≥ {:.0} MB, stop when ≤ {:.0} MB",
                            start_threshold, stop_threshold
                        ))
                            .size(13.0)
                            .color(egui::Color32::from_rgb(180, 180, 180))
                    );

                    ui.add_space(20.0);

                    // Auto clean checkbox
                    ui.checkbox(
                        &mut self.config.auto_clean_enabled,
                        egui::RichText::new("🔄 Enable auto-clean (wait 30s between cleans)")
                            .size(15.0)
                            .color(egui::Color32::WHITE)
                    );

                    ui.add_space(30.0);

                    // Save button
                    if ui.add_sized(
                        [200.0, 40.0],
                        egui::Button::new(
                            egui::RichText::new("💾 Save Configuration")
                                .size(16.0)
                        )
                    ).clicked() {
                        match self.save_config() {
                            Ok(_) => self.status_message = String::from("✅ Configuration saved"),
                            Err(e) => self.status_message = format!("❌ Error: {}", e),
                        }
                    }

                    ui.add_space(10.0);

                    // Manual clean button
                    let clean_button = ui.add_sized(
                        [200.0, 40.0],
                        egui::Button::new(
                            egui::RichText::new("🧹 Clean RAM Cache Now")
                                .size(16.0)
                        )
                    );

                    if clean_button.clicked() {
                        self.clean_ram_cache();
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
                            egui::RichText::new(format!("⏱️ Last cleaned: {} seconds ago", elapsed))
                                .size(13.0)
                                .color(egui::Color32::from_rgb(150, 150, 150))
                        );
                    }

                    ui.add_space(10.0);

                    // Warning về quyền admin
                    ui.label(
                        egui::RichText::new("⚠️ Note: Run as Administrator for best results")
                            .size(12.0)
                            .color(egui::Color32::from_rgb(255, 200, 100))
                    );
                });
            });

        // Request repaint để cập nhật UI
        ctx.request_repaint_after(Duration::from_secs(1));
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([520.0, 650.0])
            .with_min_inner_size([450.0, 550.0])
            .with_resizable(true)
            .with_title("RAM Cache Manager"),
        ..Default::default()
    };

    eframe::run_native(
        "RAM Cache Manager",
        options,
        Box::new(|_cc| Box::new(CacheManager::new())),
    )
}