mod client;
pub mod error;
pub mod model;

pub async fn get(cursor: Option<i32>, max_result: Option<i32>) -> Result<model::Response, error::Error> {
    let response = client::ApiClient::new(30, 10)?
    .get(cursor, max_result)
    .await?;

    let status: reqwest::StatusCode = response.status();
    let headers = response.headers().clone();
    let body: model::ResponseBody = response.json().await?;

    Ok(model::Response {
        body,
        status,
        headers,
    })
}
