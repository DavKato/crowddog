use crate::api::{ApiClient, ReqError, UserInfo};
use crate::settings;
use std::sync::Mutex;
use tauri::State;

fn output<T>(res: Result<T, ReqError>) -> Result<T, String> {
    match res {
        Ok(data) => Ok(data),
        Err(e) => Err(e.msg),
    }
}

#[tauri::command]
pub async fn login(
    app_handle: tauri::AppHandle,
    api: State<'_, ApiClient>,
    managed_auth: State<'_, Mutex<settings::Auth>>,
    new_auth: settings::Auth,
) -> Result<UserInfo, String> {
    {
        let mut auth = managed_auth.lock().unwrap();

        // Save the new auth
        auth.replace(&new_auth);
        auth.save(&app_handle);
    }

    // Try to login with the new auth
    output(api.login(&new_auth).await)?;
    output(api.get_user_info().await)
}
