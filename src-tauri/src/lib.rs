mod config;
mod events;
mod image;
mod thumbnail;
mod util;
mod wallpaper;

use std::process::Command;

#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_file;

use crate::{config::CONFIG, events::set_app_handle, image::Image, wallpaper::load_wallpapers};

#[tauri::command]
fn load_images() -> Result<(), String> {
    load_wallpapers()
}

#[tauri::command]
fn select_wallpaper(image: Image) -> Result<(), String> {
    println!("Setting wallpaper: {}", image.img_path);

    let current_wallpaper = CONFIG
        .current_wallpaper
        .clone()
        .ok_or("Current wallpaper path not configured")?;

    let post_script = CONFIG
        .post_script
        .clone()
        .ok_or("Post script not configured")?;

    let img_path = image.img_path.clone();

    // Spawn completely detached thread to prevent any UI blocking
    std::thread::spawn(move || {
        // Remove existing symlink/file (ignore error if doesn't exist)
        let _ = std::fs::remove_file(&current_wallpaper);

        // Create new symlink using native Rust API
        #[cfg(unix)]
        if let Err(e) = symlink(&img_path, &current_wallpaper) {
            eprintln!("Failed to create symlink: {}", e);
            return;
        }

        #[cfg(windows)]
        if let Err(e) = symlink_file(&img_path, &current_wallpaper) {
            eprintln!("Failed to create symlink: {}", e);
            return;
        }

        // Execute post script (wrapped in quotes for safety)
        match Command::new("sh")
            .arg("-c")
            .arg(format!("\"{}\"", post_script))
            .status()
        {
            Ok(status) if !status.success() => {
                eprintln!("Post script exited with non-zero status: {}", status);
            }
            Err(e) => {
                eprintln!("Failed to execute post script: {}", e);
            }
            _ => {}
        }
    });

    // Return immediately without waiting
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            set_app_handle(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![load_images, select_wallpaper])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
