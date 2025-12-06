use device_query::{DeviceQuery, DeviceState, Keycode};
use serde::Serialize;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use tauri::{Emitter, Manager, PhysicalPosition};

#[derive(Clone, Serialize)]
struct KeyEvent {
    key: String,
    event_type: String, // "press" or "release"
}

fn keycode_to_string(key: &Keycode) -> String {
    match key {
        // Modifiers
        Keycode::LAlt | Keycode::RAlt => "⌥".to_string(),
        Keycode::LShift | Keycode::RShift => "⇧".to_string(),
        Keycode::LControl | Keycode::RControl => "⌃".to_string(),
        Keycode::LMeta | Keycode::RMeta => "⌘".to_string(),
        Keycode::CapsLock => "⇪".to_string(),

        // Function keys
        Keycode::F1 => "F1".to_string(),
        Keycode::F2 => "F2".to_string(),
        Keycode::F3 => "F3".to_string(),
        Keycode::F4 => "F4".to_string(),
        Keycode::F5 => "F5".to_string(),
        Keycode::F6 => "F6".to_string(),
        Keycode::F7 => "F7".to_string(),
        Keycode::F8 => "F8".to_string(),
        Keycode::F9 => "F9".to_string(),
        Keycode::F10 => "F10".to_string(),
        Keycode::F11 => "F11".to_string(),
        Keycode::F12 => "F12".to_string(),

        // Navigation
        Keycode::Up => "↑".to_string(),
        Keycode::Down => "↓".to_string(),
        Keycode::Left => "←".to_string(),
        Keycode::Right => "→".to_string(),
        Keycode::Home => "Home".to_string(),
        Keycode::End => "End".to_string(),
        Keycode::PageUp => "PgUp".to_string(),
        Keycode::PageDown => "PgDn".to_string(),

        // Special keys
        Keycode::Space => "Space".to_string(),
        Keycode::Tab => "Tab".to_string(),
        Keycode::Enter => "Enter".to_string(),
        Keycode::Escape => "Esc".to_string(),
        Keycode::Backspace => "⌫".to_string(),
        Keycode::Delete => "Del".to_string(),
        Keycode::Insert => "Ins".to_string(),

        // Letters
        Keycode::A => "A".to_string(),
        Keycode::B => "B".to_string(),
        Keycode::C => "C".to_string(),
        Keycode::D => "D".to_string(),
        Keycode::E => "E".to_string(),
        Keycode::F => "F".to_string(),
        Keycode::G => "G".to_string(),
        Keycode::H => "H".to_string(),
        Keycode::I => "I".to_string(),
        Keycode::J => "J".to_string(),
        Keycode::K => "K".to_string(),
        Keycode::L => "L".to_string(),
        Keycode::M => "M".to_string(),
        Keycode::N => "N".to_string(),
        Keycode::O => "O".to_string(),
        Keycode::P => "P".to_string(),
        Keycode::Q => "Q".to_string(),
        Keycode::R => "R".to_string(),
        Keycode::S => "S".to_string(),
        Keycode::T => "T".to_string(),
        Keycode::U => "U".to_string(),
        Keycode::V => "V".to_string(),
        Keycode::W => "W".to_string(),
        Keycode::X => "X".to_string(),
        Keycode::Y => "Y".to_string(),
        Keycode::Z => "Z".to_string(),

        // Numbers
        Keycode::Key0 => "0".to_string(),
        Keycode::Key1 => "1".to_string(),
        Keycode::Key2 => "2".to_string(),
        Keycode::Key3 => "3".to_string(),
        Keycode::Key4 => "4".to_string(),
        Keycode::Key5 => "5".to_string(),
        Keycode::Key6 => "6".to_string(),
        Keycode::Key7 => "7".to_string(),
        Keycode::Key8 => "8".to_string(),
        Keycode::Key9 => "9".to_string(),

        // Punctuation
        Keycode::Minus => "-".to_string(),
        Keycode::Equal => "=".to_string(),
        Keycode::LeftBracket => "[".to_string(),
        Keycode::RightBracket => "]".to_string(),
        Keycode::BackSlash => "\\".to_string(),
        Keycode::Semicolon => ";".to_string(),
        Keycode::Apostrophe => "'".to_string(),
        Keycode::Comma => ",".to_string(),
        Keycode::Dot => ".".to_string(),
        Keycode::Slash => "/".to_string(),
        Keycode::Grave => "`".to_string(),

        // Numpad
        Keycode::Numpad0 => "Num0".to_string(),
        Keycode::Numpad1 => "Num1".to_string(),
        Keycode::Numpad2 => "Num2".to_string(),
        Keycode::Numpad3 => "Num3".to_string(),
        Keycode::Numpad4 => "Num4".to_string(),
        Keycode::Numpad5 => "Num5".to_string(),
        Keycode::Numpad6 => "Num6".to_string(),
        Keycode::Numpad7 => "Num7".to_string(),
        Keycode::Numpad8 => "Num8".to_string(),
        Keycode::Numpad9 => "Num9".to_string(),
        Keycode::NumpadSubtract => "Num-".to_string(),
        Keycode::NumpadAdd => "Num+".to_string(),
        Keycode::NumpadMultiply => "Num*".to_string(),
        Keycode::NumpadDivide => "Num/".to_string(),
        Keycode::NumpadEnter => "NumEnter".to_string(),

        _ => {
            let debug_name = format!("{:?}", key);
            // Handle macOS-specific key names
            match debug_name.as_str() {
                "Command" | "LCommand" | "RCommand" => "⌘".to_string(),
                "LOption" | "ROption" | "Option" => "⌥".to_string(),
                _ => debug_name,
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Position window at bottom center of screen
            if let Some(window) = app.get_webview_window("main") {
                if let Some(monitor) = window.current_monitor().ok().flatten() {
                    let screen_size = monitor.size();
                    let screen_position = monitor.position();
                    let window_size = window.outer_size().unwrap_or_default();
                    let scale_factor = monitor.scale_factor();

                    let padding_bottom = (40.0 * scale_factor) as i32; // 40px padding from bottom
                    
                    let x = screen_position.x + (screen_size.width as i32 - window_size.width as i32) / 2;
                    let y = screen_position.y + screen_size.height as i32 - window_size.height as i32 - padding_bottom;
                    
                    let _ = window.set_position(PhysicalPosition::new(x, y));
                }
                let _ = window.set_ignore_cursor_events(true);
                let _ = window.show();
            }

            // Poll keyboard state in a loop
            thread::spawn(move || {
                let device_state = DeviceState::new();
                let mut prev_keys: HashSet<Keycode> = HashSet::new();

                loop {
                    let keys: HashSet<Keycode> = device_state.get_keys().into_iter().collect();

                    // Detect newly pressed keys
                    for key in keys.difference(&prev_keys) {
                        let key_event = KeyEvent {
                            key: keycode_to_string(key),
                            event_type: "press".to_string(),
                        };
                        let _ = app_handle.emit("key-event", key_event);
                    }

                    // Detect released keys
                    for key in prev_keys.difference(&keys) {
                        let key_event = KeyEvent {
                            key: keycode_to_string(key),
                            event_type: "release".to_string(),
                        };
                        let _ = app_handle.emit("key-event", key_event);
                    }

                    prev_keys = keys;
                    thread::sleep(Duration::from_millis(10)); // ~100Hz polling
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
