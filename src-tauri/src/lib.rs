use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{Manager, WebviewWindow};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

mod config;
mod markdown_store;

struct WindowState {
    is_visible: Mutex<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub save_directory: String,
    pub filename_format: String,
    pub timestamp_heading_level: u8,
    pub timestamp_format: String,
}

impl From<config::AppConfig> for AppConfig {
    fn from(value: config::AppConfig) -> Self {
        Self {
            save_directory: value.save_directory,
            filename_format: value.filename_format,
            timestamp_heading_level: value.timestamp_heading_level,
            timestamp_format: value.timestamp_format,
        }
    }
}

impl From<AppConfig> for config::AppConfig {
    fn from(value: AppConfig) -> Self {
        Self {
            save_directory: value.save_directory,
            filename_format: value.filename_format,
            timestamp_heading_level: value.timestamp_heading_level,
            timestamp_format: value.timestamp_format,
        }
    }
}

#[tauri::command]
fn get_config(app_handle: tauri::AppHandle) -> Result<AppConfig, String> {
    config::load(&app_handle).map(AppConfig::from)
}

#[tauri::command]
fn update_config(
    new_config: AppConfig,
    app_handle: tauri::AppHandle,
) -> Result<AppConfig, String> {
    let normalized = config::AppConfig::from(new_config).normalize(&app_handle)?;
    config::save(&app_handle, &normalized)?;
    Ok(normalized.into())
}

#[tauri::command]
fn save_memo(body: String, app_handle: tauri::AppHandle) -> Result<String, String> {
    let config = config::load(&app_handle)?;
    markdown_store::append(&config, &body)
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn hide_main_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        hide_window(&window)?;
        let state = app_handle.state::<WindowState>();
        *state.is_visible.lock().map_err(|e| e.to_string())? = false;
    }
    Ok(())
}

fn hide_window(window: &WebviewWindow) -> Result<(), String> {
    window.set_always_on_top(false).map_err(|e| e.to_string())?;
    window.hide().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "error");
    }
    let toggle_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyJ);
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(WindowState {
            is_visible: Mutex::new(false),
        })
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app_handle, shortcut, event| {
                    if shortcut == &toggle_shortcut {
                        match event.state() {
                            ShortcutState::Pressed => {
                                if let Some(window) = app_handle.get_webview_window("main") {
                                    let state = app_handle.state::<WindowState>();
                                    let mut is_visible_guard = state.is_visible.lock().unwrap();
                                    if *is_visible_guard {
                                        let _ = hide_window(&window);
                                        *is_visible_guard = false;
                                    } else if let Ok(Some(monitor)) = window.current_monitor() {
                                        use tauri::{
                                            PhysicalPosition, PhysicalSize, Position, Size,
                                        };

                                        let monitor_pos = monitor.position();
                                        let monitor_size = monitor.size();

                                        let window_width = monitor_size.width / 3 * 2;
                                        let window_height = monitor_size.height / 5 * 2;

                                        let _ = window.set_size(Size::Physical(
                                            PhysicalSize::new(window_width, window_height),
                                        ));

                                        let target_x = monitor_pos.x
                                            + ((monitor_size.width as i32 - window_width as i32)
                                                / 2);
                                        let target_y = monitor_pos.y;

                                        let _ = window.set_position(Position::Physical(
                                            PhysicalPosition::new(target_x, target_y),
                                        ));

                                        let _ = window.show();
                                        let _ = window.unminimize();
                                        let _ = window.set_always_on_top(true);
                                        let _ = window.set_focus();

                                        *is_visible_guard = true;
                                    }
                                }
                            }
                            ShortcutState::Released => {}
                        }
                    }
                })
                .build(),
        )
        .setup(move |app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::GlobalShortcutExt;
                let _ = app.global_shortcut().register(toggle_shortcut);
                if let Err(err_msg) = config::load(app.handle()) {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        err_msg,
                    )));
                }
                Ok(())
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            update_config,
            save_memo,
            hide_main_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
