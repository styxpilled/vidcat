// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, fs};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn change_path(path: &str) -> Result<String, String> {
    let e = env::set_current_dir(path);

    match e {
        Ok(_) => Ok(path.to_owned()).into(),
        Err(_) => Err("Failed to set path!".to_owned()).into(),
    }
}

#[tauri::command]
fn get_path() -> String {
    env::current_dir().unwrap().to_str().unwrap().to_owned()
}

#[tauri::command]
fn rename_file(from: &str, to: &str) {
    fs::rename(from, to).unwrap()
}

#[tauri::command]
fn delete_file(path: &str) {
    fs::remove_file(path).unwrap();
}

// struct FileFetchResult {

// }

#[tauri::command]
fn get_files() -> Result<Vec<String>, String> {
    let current_path = std::env::current_dir().unwrap();
    let entries = fs::read_dir(current_path).unwrap();

    let mut result = vec![];

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                match extension.to_str().unwrap() {
                    "mp4" | "mkv" | "webm" | "mov" => {
                        result.push(path.file_name().unwrap().to_str().unwrap().to_owned())
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(result)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_path,
            change_path,
            get_files,
            delete_file,
            rename_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
