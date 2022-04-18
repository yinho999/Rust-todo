use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateJobDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub done: Option<bool>,
}
