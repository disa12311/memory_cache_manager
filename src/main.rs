// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[cfg(target_os = "windows")]
use winapi::um::memoryapi::VirtualAlloc;
#[cfg(target_os = "windows")]
use winapi::um::memoryapi::VirtualFree;
#[cfg(target_os = "windows")]
use winapi::um::sysinfoapi::{GetSystemInfo, GlobalMemoryStatusEx, MEMORYSTATUSEX, SYSTEM_INFO};
#[cfg(target_os = "windows")]
use winapi::um::winnt::{MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE};

#[derive(Default)]
struct AppState {
    config: Mutex<Config>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    start_threshold_mb: u64,
    stop_threshold_mb: u64,
    auto_clean_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            start_threshold_mb: 2048,
            stop_threshold_mb: 1024,
            auto_clean_enabled: true,
        }
    }
}

#[derive(Serialize)]
struct MemoryInfo {
    total_mb: u64,
    available_mb: u64,
    used_mb: u64,
    cache_mb: u64,
    usage_percent: f32,
}

#[cfg(target_os = "windows")]
#[tauri::command]
fn get_memory_info() -> Result<MemoryInfo, String> {
    unsafe {
        let mut mem_status: MEMORYSTATUSEX = std::mem::zeroed();
        mem_status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

        if GlobalMemoryStatusEx(&mut mem_status) == 0 {
            return Err("Failed to get memory status".to_string());
        }

        let total_mb = mem_status.ullTotalPhys / (1024 * 1024);
        let available_mb = mem_status.ullAvailPhys / (1024 * 1024);
        let used_mb = total_mb - available_mb;
        
        // Estimate cache: typically 40-60% of used memory
        let cache_mb = (used_mb as f32 * 0.5) as u64;
        let usage_percent = ((used_mb as f32 / total_mb as f32) * 100.0);

        Ok(MemoryInfo {
            total_mb,
            available_mb,
            used_mb,
            cache_mb,
            usage_percent,
        })
    }
}

#[cfg(not(target_os = "windows"))]
#[tauri::command]
fn get_memory_info() -> Result<MemoryInfo, String> {
    Err("Only supported on Windows".to_string())
}

#[cfg(target_os = "windows")]
#[tauri::command]
fn clean_memory_cache(target_mb: u64) -> Result<u64, String> {
    unsafe {
        let mut cleaned_mb: u64 = 0;
        let chunk_size = 100 * 1024 * 1024; // 100MB chunks
        let max_iterations = (target_mb * 1024 * 1024) / chunk_size;

        // Method 1: Force memory to be paged out by allocating and freeing
        for _ in 0..max_iterations {
            let ptr = VirtualAlloc(
                std::ptr::null_mut(),
                chunk_size as usize,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            );

            if !ptr.is_null() {
                // Write to memory to ensure it's committed
                std::ptr::write_bytes(ptr as *mut u8, 0, chunk_size as usize);
                
                // Free immediately
                VirtualFree(ptr, 0, MEM_RELEASE);
                cleaned_mb += 100;
            } else {
                break;
            }

            // Small delay to not overwhelm system
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        // Method 2: Clear working set of current process
        use winapi::um::processthreadsapi::GetCurrentProcess;
        use winapi::um::psapi::EmptyWorkingSet;
        
        let process = GetCurrentProcess();
        EmptyWorkingSet(process);

        Ok(cleaned_mb)
    }
}

#[cfg(not(target_os = "windows"))]
#[tauri::command]
fn clean_memory_cache(_target_mb: u64) -> Result<u64, String> {
    Err("Only supported on Windows".to_string())
}

#[tauri::command]
fn save_config(state: State<AppState>, config: Config) -> Result<(), String> {
    let mut app_config = state.config.lock().unwrap();
    *app_config = config;
    Ok(())
}

#[tauri::command]
fn load_config(state: State<AppState>) -> Result<Config, String> {
    let config = state.config.lock().unwrap();
    Ok(config.clone())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_memory_info,
            clean_memory_cache,
            save_config,
            load_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}