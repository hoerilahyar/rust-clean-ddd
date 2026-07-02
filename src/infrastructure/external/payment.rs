use reqwest::Response;

use super::HttpClient;

pub struct PaymentClient {
    http: HttpClient,
}

impl PaymentClient {
    pub fn new() -> Self {
        Self {
            http: HttpClient::new(),
        }
    }

    pub async fn get(&self, url: &str) -> Result<Response, reqwest::Error> {
        self.http.client().get(url).send().await
    }
}
