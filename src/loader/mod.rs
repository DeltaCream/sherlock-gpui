pub mod application_loader;
pub mod assets;
mod flag_loader;
mod icon_loader;
mod launcher_loader;
pub mod utils;

pub struct Loader;
pub use icon_loader::{CustomIconTheme, IconThemeGuard, resolve_icon_path};
