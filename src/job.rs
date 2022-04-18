use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
pub struct AppState {
    pub jobs: Mutex<Vec<Job>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub id: String,
    pub title: String,
    pub description: String,
    pub done: bool,
    pub time: std::time::SystemTime,
}
impl Clone for Job {
    fn clone(&self) -> Self {
        Job {
            id: self.id.clone(),
            title: self.title.clone(),
            description: self.description.clone(),
            done: self.done,
            time: self.time,
        }
    }
}

impl Job {
    pub fn new(title: String, description: String) -> Job {
        Job {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            done: false,
            time: std::time::SystemTime::now(),
        }
    }

    pub fn update(
        &mut self,
        title: Option<String>,
        description: Option<String>,
        done: Option<bool>,
    ) {
        if let Some(title) = title {
            self.title = title;
        }
        if let Some(description) = description {
            self.description = description;
        }
        if let Some(done) = done {
            self.done = done;
        }
    }
}
