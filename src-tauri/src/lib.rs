use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;
use tauri::Manager;
// Data structure for user information
#[derive(Serialize, Deserialize, Clone)]
struct UserInfo {
    name: String,
    age: u32,
}

// State to hold user data in memory
struct AppState {
    user_data: Mutex<Option<UserInfo>>,
}

#[tauri::command]
fn save_user_data(state: State<AppState>, user: UserInfo) -> Result<(), String> {
    let mut data = state.user_data.lock().unwrap();
    *data = Some(user);
    Ok(())
}

#[tauri::command]
fn get_user_data(state: State<AppState>) -> Result<Option<UserInfo>, String> {
    let data = state.user_data.lock().unwrap();
    Ok(data.clone())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            user_data: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            save_user_data,
            get_user_data,
        ])
        .setup(|app| {
            let salt_path = app
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("salt.txt");
            app.handle().plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
