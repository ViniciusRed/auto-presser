use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use enigo::{Enigo, Key, KeyboardControllable};
use tokio::time;

pub struct KeyPresser {
    running: Arc<AtomicBool>,
    bind_key: String,
    interval: f64,
    active: Arc<AtomicBool>,
}

impl KeyPresser {
    pub fn new(running: Arc<AtomicBool>, bind_key: String, interval: f64) -> Self {
        Self {
            running,
            bind_key,
            interval,
            active: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn run(&self) {
        let mut enigo = Enigo::new();

        while self.running.load(Ordering::SeqCst) {
            if self.active.load(Ordering::SeqCst) {
                if let Some(key) = self.bind_key.chars().next() {
                    enigo.key_down(Key::Layout(key));
                    enigo.key_up(Key::Layout(key));
                }
                time::sleep(Duration::from_secs_f64(self.interval)).await;
            }
            time::sleep(Duration::from_millis(50)).await;
        }
    }

    pub fn toggle(&self) {
        self.active.fetch_xor(true, Ordering::SeqCst);
    }
}