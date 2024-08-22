use crate::api::{ApiClient, Project, StopWatch, StopWatchStatus, User, WorkContent};
use crate::settings;
use crate::utils::{cancellation_token, Canceller};
use chrono::{NaiveDateTime, TimeDelta, Utc};
use std::{sync::Mutex, thread};
use tauri::{Manager, State};

#[tauri::command(rename_all = "snake_case")]
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
    api.login(&credentials).await
}

#[derive(Debug, serde::Serialize)]
pub struct InitialData {
    user: User,
    stop_watch: StopWatch,
    history: Vec<WorkContent>,
    projects: Vec<Project>,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn init_data(api: State<'_, ApiClient>) -> Result<InitialData, String> {
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
    app_handle: tauri::AppHandle,
    api: State<'_, ApiClient>,
    stop_watch: StopWatch,
    timer_handle: State<'_, TimerHandle>,
) -> Result<StopWatch, String> {
    let sw = match stop_watch.status {
        StopWatchStatus::Started => stop_watch,
        StopWatchStatus::NeedToApply => return Err("Timer is stopped without applying a work content. Fix it in the stop watch page in the CrowdLog's website.".to_string()),
        StopWatchStatus::Clean => api.start_timer(stop_watch.id).await?,
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
    api: State<'_, ApiClient>,
    stop_watch: StopWatch,
    timer_handle: State<'_, TimerHandle>,
) -> Result<StopWatch, String> {
    api.stop_timer(stop_watch.id).await?;
    api.apply_timer(stop_watch.id).await?;
    let sw = api.reset_timer(stop_watch.id).await?;

    let mut t_handle = timer_handle.lock().unwrap();
    if let Some(handle) = t_handle.take() {
        handle.cancel();
        println!("Timer stopped: {:#?}", t_handle);
    }
    Ok(sw)
}
