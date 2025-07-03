mod item;

#[tauri::command]
fn get_gun_names() -> Vec<String> {
    let names = item::get_gun_names();
    names.into()
}

#[tauri::command]
fn get_gun(gun_name: String) -> item::Gun {
    let result: item::Gun = item::get_gun(gun_name);
    result.into()
}

#[tauri::command]
fn get_ammo_names(ammo_name: String) -> Vec<String> {
    let names = item::get_ammo_names(ammo_name);
    names.into()
}

#[tauri::command]
fn get_ammo(ammo_name: String, round_name: String) -> item::Round {
    let result: item::Round = item::get_ammo(ammo_name, round_name);
    result.into()
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
        .invoke_handler(tauri::generate_handler![get_gun_names, get_gun, get_ammo, get_ammo_names])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
