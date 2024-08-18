use crate::api::{ApiClient, Project, ReqError, StopWatch, UserInfo, WorkContent};
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
) -> Result<(), String> {
    {
        let mut auth = managed_auth.lock().unwrap();

        // Save the new auth
        auth.replace(&new_auth);
        auth.save(&app_handle);
    }

    // Try to login with the new auth
    output(api.login(&new_auth).await)
}

#[derive(Debug, serde::Serialize)]
pub struct InitialData {
    user_info: UserInfo,
    stop_watch: StopWatch,
    history: Vec<WorkContent>,
    projects: Vec<Project>,
}

#[tauri::command]
pub async fn init_data(api: State<'_, ApiClient>) -> Result<InitialData, String> {
    let user_info = output(api.get_user_info().await)?;
    let stop_watch = output(api.get_stop_watch().await)?;
    let history = output(api.get_history().await)?;
    let projects = output(api.get_projects(user_info.user_id).await)?;

    Ok(InitialData {
        user_info,
        stop_watch,
        history,
        projects,
    })
}
