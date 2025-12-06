use device_query::{DeviceQuery, DeviceState, Keycode};
use serde::Serialize;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use tauri::{
    Emitter, Manager, PhysicalPosition,
    menu::{IconMenuItem, Menu, NativeIcon},
    tray::TrayIconBuilder,
    ActivationPolicy,
};

#[derive(Clone, Serialize)]
struct KeyEvent {
    key: String,
    event_type: String, // "press" or "release"
}

fn keycode_to_string(key: &Keycode, uppercase: bool) -> String {
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
        Keycode::A => if uppercase { "A" } else { "a" }.to_string(),
        Keycode::B => if uppercase { "B" } else { "b" }.to_string(),
        Keycode::C => if uppercase { "C" } else { "c" }.to_string(),
        Keycode::D => if uppercase { "D" } else { "d" }.to_string(),
        Keycode::E => if uppercase { "E" } else { "e" }.to_string(),
        Keycode::F => if uppercase { "F" } else { "f" }.to_string(),
        Keycode::G => if uppercase { "G" } else { "g" }.to_string(),
        Keycode::H => if uppercase { "H" } else { "h" }.to_string(),
        Keycode::I => if uppercase { "I" } else { "i" }.to_string(),
        Keycode::J => if uppercase { "J" } else { "j" }.to_string(),
        Keycode::K => if uppercase { "K" } else { "k" }.to_string(),
        Keycode::L => if uppercase { "L" } else { "l" }.to_string(),
        Keycode::M => if uppercase { "M" } else { "m" }.to_string(),
        Keycode::N => if uppercase { "N" } else { "n" }.to_string(),
        Keycode::O => if uppercase { "O" } else { "o" }.to_string(),
        Keycode::P => if uppercase { "P" } else { "p" }.to_string(),
        Keycode::Q => if uppercase { "Q" } else { "q" }.to_string(),
        Keycode::R => if uppercase { "R" } else { "r" }.to_string(),
        Keycode::S => if uppercase { "S" } else { "s" }.to_string(),
        Keycode::T => if uppercase { "T" } else { "t" }.to_string(),
        Keycode::U => if uppercase { "U" } else { "u" }.to_string(),
        Keycode::V => if uppercase { "V" } else { "v" }.to_string(),
        Keycode::W => if uppercase { "W" } else { "w" }.to_string(),
        Keycode::X => if uppercase { "X" } else { "x" }.to_string(),
        Keycode::Y => if uppercase { "Y" } else { "y" }.to_string(),
        Keycode::Z => if uppercase { "Z" } else { "z" }.to_string(),

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

            // Create system tray with quit menu
            let quit = IconMenuItem::with_id_and_native_icon(
                app,
                "quit",
                "Quit application",
                true,
                Some(NativeIcon::StopProgress),
                None::<&str>,
            )?;
            let menu = Menu::with_items(app, &[&quit])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| {
                    if event.id.as_ref() == "quit" {
                        app.exit(0);
                    }
                })
                .build(app)?;

            // Hide from dock (run as background app)
            app.set_activation_policy(ActivationPolicy::Accessory);

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
                let mut caps_lock_on = false;

                loop {
                    let keys: HashSet<Keycode> = device_state.get_keys().into_iter().collect();

                    // Toggle CapsLock state when CapsLock is newly pressed
                    let caps_lock_pressed = keys.contains(&Keycode::CapsLock) && !prev_keys.contains(&Keycode::CapsLock);
                    if caps_lock_pressed {
                        caps_lock_on = !caps_lock_on;
                    }

                    // Check if Shift is held
                    let shift_held = keys.contains(&Keycode::LShift) || keys.contains(&Keycode::RShift);

                    // On macOS: Shift + CapsLock = uppercase (not toggled back to lowercase)
                    let uppercase = shift_held || caps_lock_on;

                    // Detect newly pressed keys
                    for key in keys.difference(&prev_keys) {
                        let key_event = KeyEvent {
                            key: keycode_to_string(key, uppercase),
                            event_type: "press".to_string(),
                        };
                        let _ = app_handle.emit("key-event", key_event);
                    }

                    // Detect released keys
                    for key in prev_keys.difference(&keys) {
                        let key_event = KeyEvent {
                            key: keycode_to_string(key, uppercase),
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
