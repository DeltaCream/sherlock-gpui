use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AppLauncher {
    #[serde(default)]
    pub use_keywords: bool,
}
