use ndk_glue::{Event, NativeActivity, PollEvent};
use std::fs::{File, OpenOptions};
use std::io::Write;

#[no_mangle]
fn android_main() {
ndk_glue::init(/* *mut ndk_sys::ANativeActivity */, /* *mut u8 */, /* usize */, /* fn() */);

    log::info!("Starting Rust Android app");

    // Event loop to keep the app alive and responsive
    loop {
        ndk_glue::poll_events(|event| match event {
            PollEvent::Wake => {
                log::info!("Received Wake event");
            }
            PollEvent::Timeout => {
                log::info!("Poll timeout");
            }
            PollEvent::Event { event, .. } => match event {
                Event::Start => {
                    log::info!("App started");
                    write_file_to_android_data("Hello from Rust!").unwrap();
                }
                Event::Resume => {
                    log::info!("App resumed");
                }
                Event::Destroy => {
                    log::info!("App is being destroyed");
                    return;
                }
                _ => {}
            },
        });
    }
}

fn write_file_to_android_data(content: &str) -> std::io::Result<()> {
    // Get the Android app's data directory
    let android_context = ndk_glue::native_activity();
    let data_dir = android_context
        .internal_data_path()
        .to_str()
        .unwrap_or("/data");

    let file_path = format!("{}/test_file.txt", data_dir);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&file_path)?;
    file.write_all(content.as_bytes())?;

    log::info!("File written to {}", file_path);
    Ok(())
}
