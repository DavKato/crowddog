use crate::api::{
    ApiClient, Process, Project, ReqError, StopWatch, StopWatchStatus, User, WorkContent,
};
use crate::settings;
use crate::utils::{cancellation_token, Canceller};
use chrono::{NaiveDateTime, TimeDelta, Utc};
use std::{sync::Mutex, thread};
use tauri::{Manager, State};

#[tauri::command(rename_all = "snake_case")]
pub async fn login(
    credentials: settings::Credentials,
    app_handle: tauri::AppHandle,
    api: State<'_, ApiClient>,
    managed_cred: State<'_, Mutex<settings::Credentials>>,
) -> Result<(), ReqError> {
    // Try to login with the new cred
    api.login(&credentials).await?;

    {
        let mut cred = managed_cred.lock().unwrap();

        // Save the new cred
        cred.replace(&credentials);
        cred.save(&app_handle);
    }

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn re_login(
    api: State<'_, ApiClient>,
    managed_cred: State<'_, Mutex<settings::Credentials>>,
) -> Result<(), ReqError> {
    let cred = managed_cred.lock().unwrap().clone();
    api.login(&cred).await?;
    Ok(())
}

#[derive(Debug, serde::Serialize)]
pub struct InitialData {
    user: User,
    stop_watch: StopWatch,
    history: Vec<WorkContent>,
    projects: Vec<Project>,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn init_data(api: State<'_, ApiClient>) -> Result<InitialData, ReqError> {
    let user = api.get_user().await?;
    let stop_watch = api.get_stop_watch().await?;
    let history = api.get_history().await?;
    let projects = api.get_projects(user.id).await?;

    Ok(InitialData {
        user,
        stop_watch,
        history,
        projects,
    })
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_my_patterns(api: State<'_, ApiClient>) -> Result<Vec<WorkContent>, ReqError> {
    api.get_my_patterns().await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_processes(
    user_id: u32,
    project_id: u32,
    api: State<'_, ApiClient>,
) -> Result<Vec<Process>, ReqError> {
    api.get_processes(user_id, project_id).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn set_work_content(
    stop_watch: StopWatch,
    project_id: u32,
    process_id: u32,
    api: State<'_, ApiClient>,
) -> Result<WorkContent, ReqError> {
    api.add_work_content_to_history(project_id, process_id)
        .await?;
    api.set_work_content(stop_watch.id, project_id, process_id)
        .await
}

pub type TimerHandle = Mutex<Option<Canceller>>;

trait ToClockStr {
    fn to_clock_str(&self) -> String;
}
// struct Delta(TimeDelta);
impl ToClockStr for TimeDelta {
    fn to_clock_str(&self) -> String {
        let h = self.num_hours();
        let m = self.num_minutes() % 60;
        let s = self.num_seconds() % 60;
        format!("{:02}:{:02}:{:02}", h, m, s)
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn start_timer(
    stop_watch: StopWatch,
    app_handle: tauri::AppHandle,
    api: State<'_, ApiClient>,
    timer_handle: State<'_, TimerHandle>,
) -> Result<StopWatch, ReqError> {
    let sw = match stop_watch.status {
        StopWatchStatus::Started => stop_watch,
        StopWatchStatus::Clean => api.start_timer(stop_watch.id).await?,
        StopWatchStatus::NeedToApply => return Err(ReqError {
            status: reqwest::StatusCode::BAD_REQUEST.as_u16(),
            message: String::from("Timer is stopped without applying a work content. Fix it in the stop watch page in the CrowdLog's website."),
            source: None,
        }),
    };

    let mut t_handle = timer_handle.lock().unwrap();
    if t_handle.is_some() {
        return Ok(sw);
    }

    let start_at = sw.start_at.clone();
    let (canceller, token) = cancellation_token();
    t_handle.get_or_insert(canceller);
    thread::spawn(move || {
        let start = NaiveDateTime::parse_from_str(&start_at, "%Y-%m-%d %H:%M:%S").unwrap();
        loop {
            let elapsed = Utc::now().signed_duration_since(start.and_utc());
            let fmtd = elapsed.to_clock_str();
            app_handle
                .emit_all("timer_tick", Some(fmtd))
                .expect("failed to emit timer_tick event");

            if token.should_cancel() {
                break;
            }
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    Ok(sw)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn stop_timer(
    stop_watch: StopWatch,
    api: State<'_, ApiClient>,
    timer_handle: State<'_, TimerHandle>,
) -> Result<StopWatch, ReqError> {
    api.stop_timer(stop_watch.id).await?;
    api.apply_timer(stop_watch.id).await?;
    let sw = api.reset_timer(stop_watch.id).await?;

    let mut t_handle = timer_handle.lock().unwrap();
    if let Some(handle) = t_handle.take() {
        handle.cancel();
    }
    Ok(sw)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn cancel_timer(
    stop_watch: StopWatch,
    api: State<'_, ApiClient>,
    timer_handle: State<'_, TimerHandle>,
) -> Result<StopWatch, ReqError> {
    let sw = api.reset_timer(stop_watch.id).await?;

    let mut t_handle = timer_handle.lock().unwrap();
    if let Some(handle) = t_handle.take() {
        handle.cancel();
    }
    Ok(sw)
}
