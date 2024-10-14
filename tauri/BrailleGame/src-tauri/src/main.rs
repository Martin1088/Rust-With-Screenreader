// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::{thread_rng, Rng};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn dynamic() -> char {
    let mut rng = thread_rng();
    rng.gen_range(b'a'..=b'z') as char
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![dynamic])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
