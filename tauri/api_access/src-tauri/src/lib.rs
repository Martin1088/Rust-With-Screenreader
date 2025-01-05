// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/*
#[tauri::command]
fn send_to_braille_display(text: &str) {
    unsafe {
        // Convert Rust string to NSString
        let ns_string: *mut Object =
            msg_send![class!(NSString), stringWithUTF8String: text.as_ptr()];

        // Announce the string to VoiceOver, which will show it on the Braille display
        let result: bool =
            msg_send![class!(NSAccessibility), postNotificationWithIdentifier: ns_string];

        if result {
            println!("Message sent to Braille display: {}", text);
        } else {
            println!("Failed to send message to Braille display.");
        }
    }
}
*/

use core_foundation::string::CFString;
use core_foundation_sys::base::Boolean;
use objc::runtime::{Object, BOOL, YES};
use objc::{class, msg_send, sel, sel_impl};

use core_foundation::base::TCFType;
/// Announce a string to VoiceOver and Braille display

#[tauri::command]
fn send_to_voiceover_and_braille(text: &str) {
    unsafe {
        // Convert the Rust string to a CoreFoundation string
        let cf_string = CFString::new(text);

        // Create a system-wide AXUIElement
        let system_wide_element: *mut Object = msg_send![class!(AXUIElement), systemWideElement];

        // Announce the text to VoiceOver
        let result: BOOL =
            msg_send![system_wide_element, accessibilityPerformAction: cf_string.as_CFTypeRef()];
        if result == YES {
            println!("Announced to VoiceOver: {}", text);
        } else {
            println!("Failed to announce to VoiceOver.");
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            send_to_voiceover_and_braille
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
