use crate::settings;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn save_auth(
    app_handle: tauri::AppHandle,
    managed_auth: State<Mutex<settings::Auth>>,
    new_auth: settings::Auth,
) -> Result<(), String> {
    let mut auth = managed_auth.lock().unwrap();
    auth.replace(new_auth);
    auth.save(&app_handle);
    println!("Auth saved, {:?}", auth);
    Ok(())
}
