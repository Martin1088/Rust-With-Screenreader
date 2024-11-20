// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use objc::msg_send;
use objc::runtime::{Class, Object};
use rand::{thread_rng, Rng};
use std::ffi::CString;
use tauri::command;

#[command]
fn send_test(test: String) -> Result<(), String> {
    unsafe {
        let test_voiceover = Class::get("VoiceOver").ok_or("VoiceOver is not found")?;
        let vo: *mut Object = msg_send![test_voiceover, new];

        let test = CString::new(test).unwrap();
        let ns_test: *mut Object = msg_send![vo,stringWithUTF8String: test.as_ptr()];
        let _: () = msg_send![vo, output: ns_test];
        Ok(())
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn dynamic() -> char {
    let mut rng = thread_rng();
    rng.gen_range(b'a'..=b'z') as char
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![dynamic, send_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
