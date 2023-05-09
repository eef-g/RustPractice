use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;


#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    pub done: bool,

    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        Task {
            text,
            done: false,
            date: Utc::now(),
        }
    }
}