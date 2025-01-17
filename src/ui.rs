use slint::SharedString;
use std::sync::Arc;
use crate::state::AppState;
use crate::keypress::KeyPresser;

slint::slint! {
    import { Button, VerticalBox, LineEdit } from "std-widgets.slint";

    export component MainWindow inherits Window {
        title: "Auto Presser Key";
        min-width: 300px;
        min-height: 200px;

        in-out property <bool> is_running: false;
        in-out property <bool> is_enabled: true;
        in-out property <string> bind_key: "Click to bind key";
        default-font-size: 0cm;
        in-out property <string> interval: "1.0";
        in-out property <string> trigger_key: "F6";

        callback start-stop();
        callback set-trigger-key();
        callback set-bind-key();

        VerticalBox {
            padding: 10px;
            alignment: LayoutAlignment.stretch;
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
                enabled: root.is_enabled;
            }
        }
    }
}

pub fn create_ui(app_state: AppState) -> Result<MainWindow, slint::PlatformError> {
    let ui = MainWindow::new()?;
    let ui_handle = ui.as_weak();

    setup_bind_key_handler(&ui, app_state.clone());
    setup_trigger_key_handler(&ui, app_state.clone());
    setup_start_stop_handler(&ui, app_state);

    Ok(ui)
}

fn setup_bind_key_handler(ui: &MainWindow, app_state: AppState) {
    let ui_handle = ui.as_weak();
    let device_state = app_state.device_state.clone();

    ui.on_set_bind_key(move || {
        let ui = ui_handle.unwrap();
        ui.set_bind_key("Press any key...".into());

        let key = app_state.wait_for_key_press(&device_state);
        if let Some(key) = key {
            ui.set_bind_key(format!("{:?}", key).into());
        }
    });
}

fn setup_trigger_key_handler(ui: &MainWindow, app_state: AppState) {
    let ui_handle = ui.as_weak();
    let device_state = app_state.device_state.clone();

    ui.on_set_trigger_key(move || {
        let ui = ui_handle.unwrap();
        ui.set_trigger_key("Press any key...".into());

        let key = app_state.wait_for_key_press(&device_state);
        if let Some(key) = key {
            ui.set_trigger_key(format!("{:?}", key).into());
        }
    });
}

fn setup_start_stop_handler(ui: &MainWindow, app_state: AppState) {
    let ui_handle = ui.as_weak();
    let running = app_state.running.clone();

    ui.on_start_stop(move || {
        let is_running = !running.load(std::sync::atomic::Ordering::SeqCst);
        running.store(is_running, std::sync::atomic::Ordering::SeqCst);

        let ui = ui_handle.unwrap();
        ui.set_is_running(is_running);

        if is_running {
            let bind_key = ui.get_bind_key().to_string();
            let interval = ui.get_interval().to_string().parse::<f64>().unwrap_or(1.0);
            let key_presser = KeyPresser::new(running.clone(), bind_key, interval);

            tokio::spawn(async move {
                key_presser.run().await;
            });
        }
    });
}
