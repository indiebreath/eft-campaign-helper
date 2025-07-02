mod item;

#[tauri::command]
fn setup() {
    println!("{:?}", item::get_gun("AVT-40".to_string()));
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
        .invoke_handler(tauri::generate_handler![setup])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
