use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateJobDto {
    pub title: String,
    pub description: String,
}
