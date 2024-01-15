use reqwest::{Client, ClientBuilder};
use std::time::Duration;

pub struct ApiClient {
    pub client: Client,
    request_base_url: String
}

impl ApiClient {
    pub fn new(
        request_timeout_sec: u64,
        connect_timeout_sec: u64,
    ) -> Result<Self, crate::error::Error> {
        let client = ClientBuilder::default()
            .timeout(Duration::from_secs(request_timeout_sec))
            .connect_timeout(Duration::from_secs(connect_timeout_sec))
            .build()
            .map_err(crate::error::Error::Reqwest)?;

        Ok(Self {
            client,
            request_base_url: "https://connpass.com/api/v1/event/".to_owned()
        })
    }

    pub async fn get(&self, cursor: Option<i32>, max_result: Option<i32>) -> Result<reqwest::Response, crate::error::Error> {
        self.client
            .get(self.request_base_url.to_owned())
            .header("User-Agent", "connpass_api_reqwest".to_owned())
            .query(&[("order", 1), ("count", max_result.unwrap_or(100)), ("start", cursor.unwrap_or(1))])
            .send()
            .await
            .map_err(|e| e.into())
    }
}
