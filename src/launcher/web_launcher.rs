use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct WebLauncher {
    #[serde(rename = "search_engine")]
    pub engine: String,
    pub browser: Option<String>,
}
