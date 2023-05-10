use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::io::Result;
use std::path::PathBuf;

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

    pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
        todo!("Make the add_task function");
    }

    pub fn complete_task(journal_path: PathBuf, tasl_position: usize) -> Result<()> {
        todo!("Make the complete_task function");
    }

    pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
        todo!("Make the list_tasks function");
    }
}