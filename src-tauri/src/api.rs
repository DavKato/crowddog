const BASE_PATH: &str = "https://app.crowdlog.jp";

use reqwest::multipart;

use crate::settings::Auth;

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

    pub async fn login(&self, auth: &Auth) -> Result<(), String> {
        let url = format!("{}/login.cgi", BASE_PATH);
        let email = auth.email.clone();
        let passwd = auth.passwd.clone();

        let form = multipart::Form::new()
            .text("email", email)
            .text("passwd", passwd)
            .text("auto", "1")
            .text("rm", "certify");

        let req = self.client.post(url).multipart(form).build().unwrap();

        let res = self
            .client
            .execute(req)
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if res.status().is_success() {
            if let Some("result=invalid") = res.url().query() {
                return Err("Invalid email or password".to_string());
            }
            Ok(())
        } else {
            Err(format!("Failed to login: {}", res.status()))
        }
    }
}
