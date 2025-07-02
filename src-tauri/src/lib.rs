mod item;

#[tauri::command]
fn get_gun_names() -> Vec<String> {
    let names = item::get_gun_names();
    names.into()
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
        .invoke_handler(tauri::generate_handler![get_gun_names])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
