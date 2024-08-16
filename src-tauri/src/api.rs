const BASE_PATH: &str = "https://app.crowdlog.jp";

use reqwest::{multipart, StatusCode};

use crate::settings::Auth;

// custom error type
#[derive(Debug)]
pub struct ReqError {
    pub msg: String,
    pub status: Option<StatusCode>,
    pub url: Option<String>,
    pub res: Option<reqwest::Response>,
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
}
