use reqwest::Response;

use super::HttpClient;

pub struct OcrClient {
    http: HttpClient,
}

impl OcrClient {
    pub fn new() -> Self {
        Self {
            http: HttpClient::new(),
        }
    }

    pub async fn recognize(
        &self,
        url: &str,
        body: reqwest::multipart::Form,
    ) -> Result<Response, reqwest::Error> {
        self.http.client().post(url).multipart(body).send().await
    }
}
