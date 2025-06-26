#![windows_subsystem = "windows"]

use std::path::Path;
use std::process::Command;
use aria2c_browse::{url_decode};

fn open_folder_simple(path: &Path) {
    let _ = Command::new("explorer.exe")
        .arg(path)
        .spawn();
}

fn open_folder_and_select_file(path: &Path) {
    // Try to select the file, fallback to just opening the folder
    let file = path;
    let folder = file.parent().unwrap_or(file);
    let result = Command::new("explorer.exe")
        .arg("/select,")
        .arg(file)
        .spawn();
    if result.is_err() {
        open_folder_simple(folder);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // If no arguments provided, open Downloads folder
    if args.len() == 1 {
        let downloads = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users".to_string()) + "\\Downloads";
        let downloads_path = Path::new(&downloads);
        if downloads_path.exists() {
            open_folder_simple(downloads_path);
        }
        return;
    }

    // If aria2://browse/path=... provided
    if args.len() >= 2 && args[1].starts_with("aria2://browse/path=") {
        let path_part = &args[1]["aria2://browse/path=".len()..];
        let decoded_path = url_decode(path_part);
        let path = Path::new(&decoded_path);
        let final_path: std::path::PathBuf = if path.exists() {
            path.to_path_buf()
        } else {
            let downloads = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users".to_string()) + "\\Downloads";
            let downloads_path = Path::new(&downloads);
            downloads_path.to_path_buf()
        };
        if final_path.is_file() {
            open_folder_and_select_file(&final_path);
        } else {
            open_folder_simple(&final_path);
        }
        return;
    }
}