use std::{
    env::current_dir,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};

fn write_to_file(path: &Path, data: &str) -> io::Result<()> {
    // Open the file in write mode, creating it if it doesn’t exist
    let mut file = OpenOptions::new().write(true).create(true).open(path)?;

    // Write data to the file
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn write_dir(path: &Path) -> Result<(), io::Error> {
    fs::create_dir(path)
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn testdir() -> String {
    let test = current_dir().unwrap();
    let res = test.to_str().unwrap();
    let testdir = Path::new("/data/data/com.test_android.app/cert/");
    //let r = write_dir(&testdir);
    let testapp = Path::new("/data/data/com.test_android.app/cache/test.priv.pem");
    let t = write_to_file(testapp, "test Rust File");
    format!("Current dir: {}, {:?}, ", res, t)
}

#[tauri::command]
fn testenv() -> String {
    let testapp = Path::new("/data/data/com.test_android.app/cache/.env");
    let t = write_to_file(testapp, "test Rust File");
    format!("test env:  , {:?}, ", t)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, testdir, testenv])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
