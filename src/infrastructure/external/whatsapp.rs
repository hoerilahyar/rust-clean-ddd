use reqwest::Response;

use super::HttpClient;

pub struct WhatsappClient {
    http: HttpClient,
}

impl WhatsappClient {
    pub fn new() -> Self {
        Self {
            http: HttpClient::new(),
        }
    }

    pub async fn post(
        &self,
        url: &str,
        body: serde_json::Value,
    ) -> Result<Response, reqwest::Error> {
        self.http.client().post(url).json(&body).send().await
    }
}
