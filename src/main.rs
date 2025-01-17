use slint::{ModelRc, VecModel, Model};
use std::{rc::Rc, thread, time::Duration, sync::{Arc, atomic::{AtomicBool, Ordering}}};
use enigo::*;
use std::sync::Mutex;

slint::slint! {
    import { Button, ListView, LineEdit } from "std-widgets.slint";
    export struct KeyBinding {
        key: string,
        interval: int,
        active: bool,
    }

    export component MainWindow inherits Window {
        width: 300px;
        height: 400px;
        title: "Auto Presser";
        background: #1e1e1e;

        callback add_binding();
        callback toggle_binding(int);
        callback remove_binding(int);
        callback update_key(int, string);
        callback update_interval(int, int);
        in property <[KeyBinding]> bindings: [];

        VerticalLayout {
            alignment: LayoutAlignment.stretch;
            spacing: 10px;
            padding: 10px;

            Button {
                text: "Add New Binding";
                primary: true;
                clicked => { root.add_binding(); }
            }

            ListView {
                for binding[i] in bindings: Rectangle {
                    height: 80px;
                    border-radius: 4px;
                    background: binding.active ? #404040 : #2d2d2d;

                    VerticalLayout {
                        alignment: center;
                        padding: 5px;
                        spacing: 2px;

                        HorizontalLayout {
                            alignment: center;
                            spacing: 5px;
                            LineEdit { 
                                horizontal-alignment: center;
                                input-type: InputType.text;
                                width: 120px;
                                text: binding.key;
                                placeholder-text: "Press key...";
                                edited(text) => { root.update_key(i, text); }
                            }
                            LineEdit { 
                                horizontal-alignment: center;
                                width: 120px;
                                text: binding.interval;
                                placeholder-text: "Interval (ms)";
                            }
                        }

                        HorizontalLayout {
                            alignment: center;
                            spacing: 5px;
                            Button {
                                text: binding.active ? "Stop" : "Start";
                                clicked => { root.toggle_binding(i); }
                            }
                            Button {
                                text: "✖";
                                clicked => { root.remove_binding(i); }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let bindings = Rc::new(VecModel::default());
    let main_window = MainWindow::new().unwrap();
    let active_threads: Arc<Mutex<Vec<Arc<AtomicBool>>>> = Arc::new(Mutex::new(Vec::new()));

    main_window.set_bindings(ModelRc::from(bindings.clone()));

    main_window.on_add_binding({
        let bindings = bindings.clone();
        let active_threads = active_threads.clone();
        move || {
            bindings.push(KeyBinding {
                key: "a".into(),
                interval: 1000,
                active: false,
            });
            active_threads.lock().unwrap().push(Arc::new(AtomicBool::new(false)));
        }
    });

    main_window.on_update_key({
        let bindings = bindings.clone();
        move |index, new_key| {
            let index = index as usize;
            if let Some(mut binding) = bindings.row_data(index) {
                binding.key = new_key.into();
                bindings.set_row_data(index, binding);
            }
        }
    });

    main_window.on_update_interval({
        let bindings = bindings.clone();
        move |index, new_interval| {
            let index = index as usize;
            if let Some(mut binding) = bindings.row_data(index) {
                binding.interval = new_interval;
                bindings.set_row_data(index, binding);
            }
        }
    });

    main_window.on_remove_binding({
        let bindings = bindings.clone();
        let active_threads = active_threads.clone();
        move |index| {
            let index = index as usize;
            if let Some(thread_control) = active_threads.lock().unwrap().get(index) {
                thread_control.store(false, Ordering::SeqCst);
            }
            active_threads.lock().unwrap().remove(index);
            bindings.remove(index);
        }
    });

    main_window.on_toggle_binding({
        let bindings = bindings.clone();
        let active_threads = active_threads.clone();
        move |index| {
            let index = index as usize;
            let binding = bindings.row_data(index).unwrap();
            let thread_control = active_threads.lock().unwrap().get(index)
                .expect("Thread control not found")
                .clone();

            let new_active = !binding.active;
            
            let new_binding = KeyBinding {
                key: binding.key.clone(),
                interval: binding.interval,
                active: new_active,
            };
            
            bindings.set_row_data(index, new_binding.clone());
            thread_control.store(new_active, Ordering::SeqCst);

            if new_active {
                let key = binding.key.clone();
                let interval = binding.interval;
                thread::spawn(move || {
                    let mut enigo = Enigo::new();
                    while thread_control.load(Ordering::SeqCst) {
                        if let Some(key_char) = key.chars().next() {
                            enigo.key_click(Key::Layout(key_char));
                        }
                        thread::sleep(Duration::from_millis(interval as u64));
                    }
                });
            }
        }
    });

    main_window.run().unwrap();
}