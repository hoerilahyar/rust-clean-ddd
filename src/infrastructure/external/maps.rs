use reqwest::Response;

use super::HttpClient;

pub struct MapsClient {
    http: HttpClient,
}

impl MapsClient {
    pub fn new() -> Self {
        Self {
            http: HttpClient::new(),
        }
    }

    pub async fn geocode(&self, url: &str) -> Result<Response, reqwest::Error> {
        self.http.client().get(url).send().await
    }
}
