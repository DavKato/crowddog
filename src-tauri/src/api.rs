const BASE_PATH: &str = "https://app.crowdlog.jp";

use crate::settings::Credentials;
use crate::utils::today;
use reqwest::{multipart, StatusCode};

// custom error type
#[derive(Debug)]
pub struct ReqError {
    pub msg: String,
    pub status: Option<StatusCode>,
    pub url: Option<String>,
    pub res: Option<String>,
}

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
pub struct StopWatch {
    id: u32,
    start_at: String,
    status: String,
    work_content: WorkContent,
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

    async fn get(&self, url: &str, msg: &str) -> Result<reqwest::Response, ReqError> {
        let res = self.client.get(url).send().await.map_err(|e| ReqError {
            msg: e.to_string(),
            status: None,
            url: Some(url.to_string()),
            res: None,
        })?;

        if res.status().is_success() {
            Ok(res)
        } else {
            let err = Err(ReqError {
                msg: msg.to_string(),
                status: Some(res.status()),
                url: Some(url.to_string()),
                res: Some(res.text().await.unwrap()),
            });
            eprintln!("{:?}", err);
            err
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

        let res = self.client.execute(req).await.map_err(|e| ReqError {
            msg: e.to_string(),
            status: None,
            url: Some(url.to_string()),
            res: None,
        })?;

        if res.status().is_success() {
            if let Some("result=invalid") = res.url().query() {
                return Err(ReqError {
                    msg: "Invalid email or password".to_string(),
                    status: Some(StatusCode::UNAUTHORIZED),
                    url: None,
                    res: None,
                });
            };
            Ok(())
        } else {
            Err(ReqError {
                msg: "Failed to login".to_string(),
                status: Some(res.status()),
                url: Some(url.to_string()),
                res: Some(res.text().await.unwrap()),
            })
        }
    }

    pub async fn get_user(&self) -> Result<User, ReqError> {
        let url = self.url("data/page-header");
        let errmsg = "Failed to get user info";
        let res = self.get(&url, errmsg).await?;
        let text = &res.text().await.map_err(|e| ReqError {
            msg: e.to_string(),
            status: None,
            url: Some(url),
            res: None,
        })?;

        let v: PageHeader = serde_json::from_str(text).map_err(|e| ReqError {
            msg: e.to_string(),
            status: None,
            url: None,
            res: None,
        })?;

        Ok(User {
            id: v.user_info.user_id,
            email: v.user_info.email,
            name: format!("{} {}", v.user_info.family_name, v.user_info.first_name),
        })
    }

    pub async fn get_stop_watch(&self) -> Result<StopWatch, ReqError> {
        let url = self.url("apis/my/stop_watches");
        let errmsg = "Failed to get stop watches";
        let res = self.get(&url, errmsg).await?;
        let text = &res.text().await.map_err(|e| ReqError {
            msg: e.to_string(),
            status: None,
            url: Some(url),
            res: None,
        })?;

        let v: StopWatchResponse = serde_json::from_str(text).map_err(|e| ReqError {
            msg: e.to_string(),
            status: None,
            url: None,
            res: None,
        })?;

        Ok(v.stop_watches[0].clone())
    }

    pub async fn get_history(&self) -> Result<Vec<WorkContent>, ReqError> {
        let url = self.url(format!("apis/my/histories/work_contents?date={}", today()).as_str());
        let errmsg = "Failed to get history";
        let res = self.get(&url, errmsg).await?;
        let text = &res.text().await.map_err(|e| ReqError {
            msg: e.to_string(),
            status: None,
            url: Some(url),
            res: None,
        })?;

        let v: HistoryResponse = serde_json::from_str(text).map_err(|e| ReqError {
            msg: e.to_string(),
            status: None,
            url: None,
            res: None,
        })?;

        let flattened = v
            .work_content_histories
            .iter()
            .map(|x| x.work_content.clone())
            .collect::<Vec<WorkContent>>();

        Ok(flattened)
    }

    pub async fn get_projects(&self, user_id: u32) -> Result<Vec<Project>, ReqError> {
        let url = self.url(format!("apis/users/{}/projects?date={}", user_id, today()).as_str());
        let errmsg = "Failed to get projects";
        let res = self.get(&url, errmsg).await?;
        let text = &res.text().await.map_err(|e| ReqError {
            msg: e.to_string(),
            status: None,
            url: Some(url),
            res: None,
        })?;

        let v: ProjectsResponse = serde_json::from_str(text).map_err(|e| ReqError {
            msg: e.to_string(),
            status: None,
            url: None,
            res: None,
        })?;

        Ok(v.projects)
    }
}
