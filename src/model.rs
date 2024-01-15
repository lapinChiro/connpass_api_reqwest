use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Response {
    pub body: ResponseBody,
    pub status: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody {
    pub events: Vec<Event>,
    pub results_available: i64,
    pub results_returned: i64,
    pub results_start: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub accepted: Option<i64>,
    pub address: Option<String>,
    pub catch: Option<String>,
    pub description: Option<String>,
    pub ended_at: Option<String>,
    pub event_id: Option<i64>,
    pub event_type: Option<String>,
    pub event_url: Option<String>,
    pub hash_tag: Option<String>,
    pub lat: Option<String>,
    pub limit: Option<i64>,
    pub lon: Option<String>,
    pub owner_display_name: Option<String>,
    pub owner_nickname: Option<String>,
    pub owner_id: Option<i64>,
    pub place: Option<String>,
    pub started_at: Option<String>,
    pub title: Option<String>,
    pub updated_at: Option<String>,
    pub waiting: Option<i64>,
    pub series: Option<Series>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Series {
    pub id: i64,
    pub title: String,
    pub url: String,
}