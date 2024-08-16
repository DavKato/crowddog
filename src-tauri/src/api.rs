const BASE_PATH: &str = "https://app.crowdlog.jp";

use crate::settings::Auth;
use reqwest::{multipart, StatusCode};

// custom error type
#[derive(Debug)]
pub struct ReqError {
    pub msg: String,
    pub status: Option<StatusCode>,
    pub url: Option<String>,
    pub res: Option<reqwest::Response>,
}

#[derive(Debug, serde::Deserialize)]
struct PageHeader {
    user_info: ServerUserInfo,
}

#[derive(Debug, serde::Deserialize)]
struct ServerUserInfo {
    user_id: u32,
    email: String,
    first_name: String,
    family_name: String,
}

#[derive(Debug, serde::Serialize)]
pub struct UserInfo {
    user_id: u32,
    email: String,
    name: String,
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
                res: Some(res),
            });
            eprintln!("{:?}", err);
            err
        }
    }

    pub async fn login(&self, auth: &Auth) -> Result<(), ReqError> {
        let url = self.url("login.cgi");
        let email = auth.email.clone();
        let passwd = auth.passwd.clone();

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
                res: Some(res),
            })
        }
    }

    pub async fn get_user_info(&self) -> Result<UserInfo, ReqError> {
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

        Ok(UserInfo {
            user_id: v.user_info.user_id,
            email: v.user_info.email,
            name: format!("{} {}", v.user_info.family_name, v.user_info.first_name),
        })
    }
}
