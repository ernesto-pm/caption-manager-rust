#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone)]
#[serde(default)]
pub struct Dataset {
    pub name: String,
    pub directory_abs_path: String,
}

impl Default for Dataset {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            directory_abs_path: "".to_owned()
        }
    }
}