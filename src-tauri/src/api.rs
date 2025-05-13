const BASE_PATH: &str = "https://app.crowdlog.jp";

use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

use crate::settings::Credentials;
use crate::utils::{log_if_error, today};
use reqwest::multipart;

// custom error type
#[derive(Debug, serde::Serialize)]
pub struct ReqError {
    pub status: u16,
    pub message: String,
    pub source: Option<String>,
}
impl ReqError {
    pub fn from(err: &reqwest::Error) -> Self {
        let status = match err.status() {
            Some(v) => v.into(),
            None => 500,
        };

        let source = err.source().map(|v| v.to_string());

        Self {
            status,
            message: err.to_string(),
            source,
        }
    }
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
pub struct WorkContentResponse {
    work_content: WorkContent,
}

pub struct ApiClient {
    client: reqwest::Client,
}
impl ApiClient {
    pub fn new() -> Self {
        let client = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .referer(false)
            .http2_keep_alive_interval(Duration::from_secs(60 * 50))
            .build()
            .unwrap();
        Self { client }
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", BASE_PATH, path)
    }

    async fn adapt(&self, res: Result<reqwest::Response, reqwest::Error>) -> Result<(), ReqError> {
        let response = log_if_error(res)?;

        match response.error_for_status() {
            Ok(_res) => Ok(()),
            Err(err) => {
                eprintln!("{:#?}", &err);
                Err(ReqError::from(&err))
            }
        }
        // if response.status().is_success() {
        //     Ok(())
        // } else {
        //     let url = response.url().clone();
        //     {
        //         eprintln!("failed to fetch: {:#?}", &response);
        //     }
        //     eprintln!("message: {:#?}", response.text().await.unwrap());
        //     Err(ReqError::new(response))
        // }
    }

    async fn adapt_json<T: serde::de::DeserializeOwned>(
        &self,
        res: Result<reqwest::Response, reqwest::Error>,
    ) -> Result<T, ReqError> {
        let response = log_if_error(res)?;

        match response.error_for_status() {
            Ok(res) => {
                let body = log_if_error(res.json::<T>().await)?;
                Ok(body)
            }
            Err(err) => {
                eprintln!("{:#?}", &err);
                Err(ReqError::from(&err))
            }
        }

        // if response.status().is_success() {
        // } else {
        //     let url = response.url().clone();
        //     {
        //         eprintln!("failed to fetch: {:#?}", &response);
        //     }
        //     eprintln!("message: {:#?}", response.text().await.unwrap());
        //     Err(format!("failed to fetch: {}", url))
        // }
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
            let err = ReqError {
                status: reqwest::StatusCode::BAD_REQUEST.as_u16(),
                message: String::from("Invalid email or password"),
                source: None,
            };
            eprintln!("{:#?}", &err);
            return Err(err);
        };

        self.adapt(res).await?;

        Ok(())
    }

    pub async fn get_user(&self) -> Result<User, ReqError> {
        let url = self.url("data/page-header");

        #[derive(Debug, serde::Deserialize)]
        struct ServerUser {
            user_id: u32,
            email: String,
            first_name: String,
            family_name: String,
        }
        #[derive(Debug, serde::Deserialize)]
        struct Response {
            user_info: ServerUser,
        }

        let res = self.client.get(url).send().await;
        let v: Response = self.adapt_json(res).await?;

        Ok(User {
            id: v.user_info.user_id,
            email: v.user_info.email,
            name: format!("{} {}", v.user_info.family_name, v.user_info.first_name),
        })
    }

    pub async fn get_stop_watch(&self) -> Result<StopWatch, ReqError> {
        let url = self.url("apis/my/stop_watches");

        #[derive(Debug, serde::Deserialize)]
        struct Response {
            stop_watches: Vec<StopWatch>,
        }

        let res = self.client.get(url).send().await;
        let v: Response = self.adapt_json(res).await?;

        Ok(v.stop_watches[0].clone())
    }

    pub async fn get_history(&self) -> Result<Vec<WorkContent>, ReqError> {
        let url = self.url(
            format!(
                "apis/my/histories/work_contents?date={}&active=true",
                today()
            )
            .as_str(),
        );

        #[derive(Debug, serde::Deserialize)]
        struct Response {
            work_content_histories: Vec<WorkContentResponse>,
        }
        let res = self.client.get(url).send().await;
        let v: Response = self.adapt_json(res).await?;

        let flattened = v
            .work_content_histories
            .iter()
            .map(|x| x.work_content.clone())
            .collect::<Vec<WorkContent>>();

        Ok(flattened)
    }

    pub async fn get_my_patterns(&self) -> Result<Vec<WorkContent>, ReqError> {
        let url = self.url(format!("apis/my/my_patterns/?date={}&active=true", today()).as_str());

        #[derive(Debug, serde::Deserialize)]
        struct Response {
            my_patterns: Vec<WorkContentResponse>,
        }

        let res = self.client.get(url).send().await;
        let v: Response = self.adapt_json(res).await?;

        let flattened = v
            .my_patterns
            .iter()
            .map(|x| x.work_content.clone())
            .collect::<Vec<WorkContent>>();

        Ok(flattened)
    }

    pub async fn get_projects(&self, user_id: u32) -> Result<Vec<Project>, ReqError> {
        let url = self
            .url(format!("apis/users/{user_id}/projects?date={}&active=true", today()).as_str());

        #[derive(Debug, serde::Deserialize)]
        struct Response {
            projects: Vec<Project>,
        }

        let res = self.client.get(url).send().await;
        let v: Response = self.adapt_json(res).await?;

        Ok(v.projects)
    }

    pub async fn get_processes(
        &self,
        user_id: u32,
        project_id: u32,
    ) -> Result<Vec<Process>, ReqError> {
        let url = self.url(format!("apis/users/{user_id}/projects/{project_id}/processes/level/1/?active=true&per_page=100").as_str());

        #[derive(Debug, serde::Deserialize)]
        struct Response {
            processes: Vec<Process>,
        }

        let res = self.client.get(url).send().await;
        let v: Response = self.adapt_json(res).await?;

        Ok(v.processes)
    }

    pub async fn set_work_content(
        &self,
        sw_id: u32,
        project_id: u32,
        process_id: u32,
    ) -> Result<WorkContent, ReqError> {
        let url = self.url(format!("apis/my/stop_watches/{sw_id}/").as_str());

        #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
        struct Body {
            project_no: u32,
            process_1: u32,
            memo: String,
        }
        let body = Body {
            project_no: project_id,
            process_1: process_id,
            memo: String::new(),
        };

        let res = self.client.put(url).json(&body).send().await;
        let v: WorkContentResponse = self.adapt_json(res).await?;

        Ok(v.work_content)
    }

    pub async fn add_work_content_to_history(
        &self,
        project_id: u32,
        process_id: u32,
    ) -> Result<(), ReqError> {
        let url = self.url("apis/my/histories/work_contents/");

        let mut body = HashMap::new();
        body.insert("project_id", project_id);
        body.insert("process_id", process_id);

        let res = self.client.put(url).json(&body).send().await;
        self.adapt(res).await?;

        Ok(())
    }

    pub async fn start_timer(&self, sw_id: u32) -> Result<StopWatch, ReqError> {
        let url = self.url(format!("apis/my/stop_watches/{sw_id}/start/").as_str());

        let mut body = HashMap::new();
        body.insert("ignore_id", sw_id);

        let res = self.client.put(url).json(&body).send().await;
        let v: StopWatch = self.adapt_json(res).await?;

        Ok(v)
    }

    pub async fn stop_timer(&self, sw_id: u32) -> Result<StopWatch, ReqError> {
        let url = self.url(format!("apis/my/stop_watches/{sw_id}/stop/").as_str());

        let res = self.client.put(url).send().await;
        let v: StopWatch = self.adapt_json(res).await?;

        Ok(v)
    }

    pub async fn apply_timer(&self, sw_id: u32) -> Result<(), ReqError> {
        let url = self.url("apis/my/timesheets/stop_watch_apply/");

        let mut body = HashMap::new();
        body.insert("id", sw_id);

        let res = self.client.post(url).json(&body).send().await;
        self.adapt(res).await?;

        Ok(())
    }

    pub async fn reset_timer(&self, sw_id: u32) -> Result<StopWatch, ReqError> {
        let url = self.url(format!("apis/my/stop_watches/{sw_id}/reset/").as_str());

        let res = self.client.put(url).send().await;
        let v: StopWatch = self.adapt_json(res).await?;

        Ok(v)
    }
}

// Need to handle status code 401 to auto login and then retry the request
