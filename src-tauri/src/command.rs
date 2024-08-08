use crate::api::ApiClient;
use crate::settings;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub async fn login(
    app_handle: tauri::AppHandle,
    api: State<'_, ApiClient>,
    managed_auth: State<'_, Mutex<settings::Auth>>,
    new_auth: settings::Auth,
) -> Result<(), String> {
    {
        let mut auth = managed_auth.lock().unwrap();

        // Save the new auth
        auth.replace(&new_auth);
        auth.save(&app_handle);
    }

    // Try to login with the new auth
    api.login(&new_auth).await
}
