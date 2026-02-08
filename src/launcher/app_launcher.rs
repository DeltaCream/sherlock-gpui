use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AppLauncher {
    pub use_keywords: bool,
}
