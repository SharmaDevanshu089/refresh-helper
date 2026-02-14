use tauri::menu::Menu;
use tauri::menu::MenuItem;
use tauri::tray::TrayIconBuilder;
use windows::core::*;
use windows::Win32::Graphics::Gdi::*;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[derive(Debug, serde::Serialize, Clone)]
pub struct CurrentDisplayInfo {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub bit_rate: u32,
}
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            println!("Hello World");
            let menu_quit = MenuItem::with_id(app, "quit", "Quit Program", true, None::<&str>)?;
            let create_window_menu =
                MenuItem::with_id(app, "create_gui_window", "Options", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&create_window_menu, &menu_quit])?;
            let _tray = TrayIconBuilder::new()
                .menu(&tray_menu)
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn get_current_display() -> std::result::Result<CurrentDisplayInfo, String> {
    unsafe {
        let device_name = w!("\\\\.\\DISPLAY1");
        let mut dev_mode = DEVMODEW::default();
        dev_mode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;
        let setting_boolean = EnumDisplaySettingsExW(
            PCWSTR(device_name.as_ptr()),
            ENUM_CURRENT_SETTINGS,
            &mut dev_mode,
            ENUM_DISPLAY_SETTINGS_FLAGS(0),
        )
        .as_bool();

        if setting_boolean {
            Ok(CurrentDisplayInfo {
                width: dev_mode.dmPelsWidth,
                height: dev_mode.dmPelsHeight,
                refresh_rate: dev_mode.dmDisplayFrequency,
                bit_rate: dev_mode.dmBitsPerPel,
            })
        } else {
            Err("Failed to Read Current Display".to_string())
        }
    }
}
