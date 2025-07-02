mod item;

#[tauri::command]
fn json_to_front() -> item::Gun {
    item::assault_to_json().into()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![json_to_front])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
