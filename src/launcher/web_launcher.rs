use gpui::SharedString;

#[derive(Clone, Debug)]
pub struct WebLauncher {
    pub engine: String,
    pub browser: Option<String>,
}
