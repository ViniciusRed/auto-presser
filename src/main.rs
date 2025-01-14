use slint::SharedString;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::time;
use enigo::{Enigo, Key, KeyboardControllable};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread;
use rdev::{listen, Event, EventType};

slint::slint! {
    import { Button, VerticalBox, LineEdit } from "std-widgets.slint";

    export component MainWindow inherits Window {
        title: "Auto Presser Key";
        min-width: 300px;
        min-height: 200px;

        in-out property <bool> is_running: false;
        in-out property <string> bind_key: "Click to bind key";
        in-out property <string> interval: "1.0";
        in-out property <string> trigger_key: "F6";

        callback start-stop();
        callback set-trigger-key();
        callback set-bind-key();

        VerticalBox {
            padding: 10px;
            spacing: 10px;

            Button {
                text: "Current bind key: " + root.bind_key;
                clicked => { set-bind-key(); }
            }

            LineEdit {
                placeholder-text: "Interval in seconds";
                text <=> root.interval;
            }

            Button {
                text: "Current trigger key: " + root.trigger_key;
                clicked => { set-trigger-key(); }
            }

            Button {
                text: root.is_running ? "Stop" : "Start";
                clicked => { start-stop(); }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();
    let device_state = DeviceState::new();
    
    let running = Arc::new(AtomicBool::new(false));
    let running_clone = running.clone();

    // Clone values for bind key closure
    let ui_handle_bind = ui_handle.clone();
    let device_state_bind = device_state.clone();
    ui.on_set_bind_key(move || {
        let ui = ui_handle_bind.unwrap();
        println!("Press any key to bind...");
        
        while device_state_bind.get_keys().is_empty() {
            std::thread::sleep(Duration::from_millis(50));
        }
        let keys = device_state_bind.get_keys();
        if let Some(key) = keys.first() {
            ui.set_bind_key(format!("{:?}", key).into());
        }
    });

    // Clone values for trigger key closure
    let ui_handle_trigger = ui_handle.clone();
    let device_state_trigger = device_state.clone();
    ui.on_set_trigger_key(move || {
        let ui = ui_handle_trigger.unwrap();
        println!("Press any key to set as trigger...");
        
        while device_state_trigger.get_keys().is_empty() {
            std::thread::sleep(Duration::from_millis(50));
        }
        let keys = device_state_trigger.get_keys();
        if let Some(key) = keys.first() {
            ui.set_trigger_key(format!("{:?}", key).into());
        }
    });
    
    ui.on_start_stop(move || {
        let is_running = !running.load(Ordering::SeqCst);
        running.store(is_running, Ordering::SeqCst);
        
        let ui = ui_handle.unwrap();
        ui.set_is_running(is_running);

        if is_running {
            let bind_key = ui.get_bind_key().to_string();
            let interval = ui.get_interval().to_string().parse::<f64>().unwrap_or(1.0);
            let running_for_task = running_clone.clone();
            
            tokio::spawn(async move {
                let mut enigo = Enigo::new();
                while running_for_task.load(Ordering::SeqCst) {
                    match bind_key.as_str() {
                        "F6" => enigo.key_click(Key::F6),
                        _ => if let Some(c) = bind_key.chars().next() {
                            enigo.key_click(Key::Layout(c))
                        }
                    }
                    time::sleep(Duration::from_secs_f64(interval)).await;
                }
            });
        }
    });
    ui.run().unwrap();
}