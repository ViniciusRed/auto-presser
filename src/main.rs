mod ui;
mod keypress;
mod state;

use state::AppState;
use slint::ComponentHandle;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::thread;

#[cfg(target_os = "windows")]
use winapi::um::wincon::GetConsoleWindow;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{ShowWindow, SW_HIDE};

fn main() {
    // Remove tokio to improve Wine compatibility
    #[cfg(target_os = "windows")]
    hide_console_window();

    let app_state = AppState::new();
    let running = app_state.running.clone();
    
    thread::spawn(move || {
        let device_state = DeviceState::new();
        while running.load(Ordering::SeqCst) {
            if let Some(key) = device_state.get_keys().first() {
                if *key == Keycode::F6 {
                    running.fetch_xor(true, Ordering::SeqCst);
                    thread::sleep(Duration::from_millis(200));
                }
            }
            thread::sleep(Duration::from_millis(50));
        }
    });

    let ui = ui::create_ui(app_state).expect("Failed to create UI");
    ui.run().unwrap();
}

#[cfg(target_os = "windows")]
fn hide_console_window() {
    unsafe {
        let window = GetConsoleWindow();
        if !window.is_null() {
            ShowWindow(window, SW_HIDE);
        }
    }
}
