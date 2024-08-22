const BASE_PATH: &str = "https://app.crowdlog.jp";

use std::collections::HashMap;

use crate::settings::Credentials;
use crate::utils::{log_if_error, today};
use reqwest::multipart;

// custom error type
pub type ReqError = String;

#[derive(Debug, serde::Deserialize)]
struct PageHeader {
    user_info: ServerUser,
}

#[derive(Debug, serde::Deserialize)]
struct ServerUser {
    user_id: u32,
    email: String,
    first_name: String,
    family_name: String,
}

#[derive(Debug, serde::Serialize)]
pub struct User {
    pub id: u32,
    pub email: String,
    pub name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Project {
    id: u32,
    name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Process {
    id: u32,
    name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct WorkContent {
    project: Option<Project>,
    process: Option<Process>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum StopWatchStatus {
    #[serde(rename = "timing")]
    Started,
    #[serde(rename = "stopped")]
    NeedToApply,
    #[serde(rename = "reset")]
    Clean,
}
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct StopWatch {
    pub id: u32,
    pub start_at: String,
    pub status: StopWatchStatus,
    pub work_content: WorkContent,
}

#[derive(Debug, serde::Deserialize)]
struct StopWatchResponse {
    stop_watches: Vec<StopWatch>,
}

#[derive(Debug, serde::Deserialize)]
struct ProjectsResponse {
    projects: Vec<Project>,
}

#[derive(Debug, serde::Deserialize)]
pub struct WorkContentHistory {
    work_content: WorkContent,
}

#[derive(Debug, serde::Deserialize)]
struct HistoryResponse {
    work_content_histories: Vec<WorkContentHistory>,
}
pub struct ApiClient {
    client: reqwest::Client,
}
impl ApiClient {
    pub fn new() -> Self {
        let client = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .build()
            .unwrap();
        Self { client }
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", BASE_PATH, path)
    }

    async fn adapt(&self, res: Result<reqwest::Response, reqwest::Error>) -> Result<(), ReqError> {
        let response = log_if_error(res)?;

        if response.status().is_success() {
            Ok(())
        } else {
            let url = response.url().clone();
            {
                eprintln!("failed to fetch: {:#?}", &response);
            }
            eprintln!("message: {:#?}", response.text().await.unwrap());
            Err(format!("failed to fetch: {}", url))
        }
    }

    async fn adapt_json<T: serde::de::DeserializeOwned>(
        &self,
        res: Result<reqwest::Response, reqwest::Error>,
    ) -> Result<T, ReqError> {
        let response = log_if_error(res)?;

        if response.status().is_success() {
            let url = response.url().clone();
            let body = log_if_error(response.json::<T>().await).map_err(|e| {
                eprintln!("Failed to parse the response from {};", url);
                e
            })?;
            Ok(body)
        } else {
            let url = response.url().clone();
            {
                eprintln!("failed to fetch: {:#?}", &response);
            }
            eprintln!("message: {:#?}", response.text().await.unwrap());
            Err(format!("failed to fetch: {}", url))
        }
    }

    pub async fn login(&self, cred: &Credentials) -> Result<(), ReqError> {
        let url = self.url("login.cgi");
        let email = cred.email.clone();
        let passwd = cred.passwd.clone();

        let form = multipart::Form::new()
            .text("email", email)
            .text("passwd", passwd)
            .text("auto", "1")
            .text("rm", "certify");

        let req = self.client.post(&url).multipart(form).build().unwrap();
        let res = self.client.execute(req).await;

        let url = res
            .as_ref()
            .map_or_else(|_| None, |v| v.url().query().map(|x| x.to_string()));

        if url == Some("result=invalid".to_string()) {
            let err = Err("Invalid email or password");
            return log_if_error(err);
        };

        self.adapt(res)
            .await
            .map_err(|_| "Failed to login".to_string())?;

        Ok(())
    }

    pub async fn get_user(&self) -> Result<User, ReqError> {
        let url = self.url("data/page-header");

        let res = self.client.get(url).send().await;
        let v: PageHeader = self
            .adapt_json(res)
            .await
            .map_err(|_| "Failed to get user info".to_string())?;

        Ok(User {
            id: v.user_info.user_id,
            email: v.user_info.email,
            name: format!("{} {}", v.user_info.family_name, v.user_info.first_name),
        })
    }

    pub async fn get_stop_watch(&self) -> Result<StopWatch, ReqError> {
        let url = self.url("apis/my/stop_watches");

        let res = self.client.get(url).send().await;
        let v: StopWatchResponse = self
            .adapt_json(res)
            .await
            .map_err(|_| "Failed to get the stop watch data".to_string())?;

        Ok(v.stop_watches[0].clone())
    }

    pub async fn get_history(&self) -> Result<Vec<WorkContent>, ReqError> {
        let url = self.url(format!("apis/my/histories/work_contents?date={}", today()).as_str());

        let res = self.client.get(url).send().await;
        let v: HistoryResponse = self
            .adapt_json(res)
            .await
            .map_err(|_| "Failed to get history".to_string())?;
        let flattened = v
            .work_content_histories
            .iter()
            .map(|x| x.work_content.clone())
            .collect::<Vec<WorkContent>>();

        Ok(flattened)
    }

    pub async fn get_projects(&self, user_id: u32) -> Result<Vec<Project>, ReqError> {
        let url = self.url(format!("apis/users/{}/projects?date={}", user_id, today()).as_str());

        let res = self.client.get(url).send().await;
        let v: ProjectsResponse = self
            .adapt_json(res)
            .await
            .map_err(|_| "Failed to get projects".to_string())?;

        Ok(v.projects)
    }

    pub async fn start_timer(&self, sw_id: u32) -> Result<StopWatch, ReqError> {
        let url = self.url(format!("apis/my/stop_watches/{}/start/", sw_id).as_str());

        let mut body = HashMap::new();
        body.insert("ignore_id", sw_id);

        let res = self.client.put(url).json(&body).send().await;
        let v: StopWatch = self
            .adapt_json(res)
            .await
            .map_err(|_| "Failed to start the timer".to_string())?;

        Ok(v)
    }

    pub async fn stop_timer(&self, sw_id: u32) -> Result<StopWatch, ReqError> {
        let url = self.url(format!("apis/my/stop_watches/{}/stop/", sw_id).as_str());

        let res = self.client.put(url).send().await;
        let v: StopWatch = self
            .adapt_json(res)
            .await
            .map_err(|_| "Failed to stop the timer".to_string())?;

        Ok(v)
    }

    pub async fn apply_timer(&self, sw_id: u32) -> Result<(), ReqError> {
        let url = self.url("apis/my/timesheets/stop_watch_apply/");

        let mut body = HashMap::new();
        body.insert("id", sw_id);

        let res = self.client.post(url).json(&body).send().await;
        self.adapt(res)
            .await
            .map_err(|_| "Failed to apply the timer".to_string())?;

        Ok(())
    }

    pub async fn reset_timer(&self, sw_id: u32) -> Result<StopWatch, ReqError> {
        let url = self.url(format!("apis/my/stop_watches/{}/reset/", sw_id).as_str());

        let res = self.client.put(url).send().await;
        let v: StopWatch = self
            .adapt_json(res)
            .await
            .map_err(|_| "Failed to reset the timer".to_string())?;

        Ok(v)
    }
}
