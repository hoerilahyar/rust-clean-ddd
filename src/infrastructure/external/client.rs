use std::time::Duration;

use reqwest::{Client, ClientBuilder};

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = ClientBuilder::new()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .pool_idle_timeout(Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .build()
            .expect("failed to create http client");

        Self { client }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
