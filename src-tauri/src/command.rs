use crate::api::{ApiClient, Project, ReqError, StopWatch, User, WorkContent};
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
    managed_cred: State<'_, Mutex<settings::Credentials>>,
    credentials: settings::Credentials,
) -> Result<(), String> {
    {
        let mut cred = managed_cred.lock().unwrap();

        // Save the new cred
        cred.replace(&credentials);
        cred.save(&app_handle);
    }

    // Try to login with the new cred
    output(api.login(&credentials).await)
}

#[derive(Debug, serde::Serialize)]
pub struct InitialData {
    user: User,
    stop_watch: StopWatch,
    history: Vec<WorkContent>,
    projects: Vec<Project>,
}

#[tauri::command]
pub async fn init_data(api: State<'_, ApiClient>) -> Result<InitialData, String> {
    let user = output(api.get_user().await)?;
    let stop_watch = output(api.get_stop_watch().await)?;
    let history = output(api.get_history().await)?;
    let projects = output(api.get_projects(user.id).await)?;

    Ok(InitialData {
        user,
        stop_watch,
        history,
        projects,
    })
}
