use std::process::Command;

use tauri::{AppHandle, Emitter};

use resolve_path::PathResolveExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    pub name: String,
    pub path: String,
}

#[tauri::command]
fn load_images(app_handle: AppHandle) -> Result<(), String> {
    let exts = ["png", "jpg", "jpeg", "webp"];

    std::thread::spawn(move || {
        let path_str = "~/Pictures/wallpapers/";
        let absolute_path = path_str
            .try_resolve()
            .map_err(|e| format!("Failed to resolve path: {}", e))
            .ok()
            .unwrap();

        if let Ok(entries) = std::fs::read_dir(absolute_path) {
            for entry in entries.flatten() {
                if !entry.file_type().map(|f| f.is_file()).unwrap_or(false) {
                    continue;
                }

                let ext = entry
                    .path()
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                if !exts.contains(&ext.as_str()) {
                    continue;
                }

                let image = Image {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry.path().to_string_lossy().to_string(),
                };

                let _ = app_handle.emit("new-image", &image);
            }
        }
    });

    Ok(())
}

#[tauri::command]
fn select_wallpaper(image: Image) -> Result<(), String> {
    println!("Setting wallpaper: {}", image.path);

    let status = Command::new("awww") // binary
        .arg("img") // first argument
        .arg(&image.path) // second argument
        .arg("--transition-type") // option
        .arg("random") // option value
        .status() // run command and get status
        .map_err(|e| format!("Failed to run command: {}", e))?;

    if !status.success() {
        return Err(format!("Command exited with status: {}", status));
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_images, select_wallpaper])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
