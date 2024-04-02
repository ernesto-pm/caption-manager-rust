use crate::models::Dataset;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppState {
    pub datasets: Vec<Dataset>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            datasets: vec![]
        }
    }
}