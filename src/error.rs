use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("serde json {0}")]
    Json(#[from] serde_json::Error),

    #[error("reqwest {0}")]
    Reqwest(#[from] reqwest::Error),
}
