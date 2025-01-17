use device_query::{DeviceQuery, DeviceState, Keycode};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

#[derive(Clone)]
pub struct AppState {
    pub running: Arc<AtomicBool>,
    pub device_state: Arc<DeviceState>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(true)), // Start with true to enable monitoring
            device_state: Arc::new(DeviceState::new()),
        }
    }

    pub fn toggle_running(&self) {
        self.running.fetch_xor(true, Ordering::SeqCst);
    }

    pub fn wait_for_key_press(&self, device_state: &DeviceState) -> Option<Keycode> {
        while device_state.get_keys().is_empty() {
            std::thread::sleep(Duration::from_millis(50));
        }

        let key = device_state.get_keys().first().cloned();

        // Wait until key is released
        while !device_state.get_keys().is_empty() {
            std::thread::sleep(Duration::from_millis(50));
        }

        key
    }
}
