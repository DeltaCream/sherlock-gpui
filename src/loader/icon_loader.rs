use linicon::lookup_icon;

use crate::utils::errors::{SherlockError, SherlockErrorType};
use crate::utils::files::home_dir;
use crate::{ICONS, sherlock_error};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct CustomIconTheme {
    pub buf: HashMap<String, Option<Arc<Path>>>,
}
impl CustomIconTheme {
    pub fn new() -> Self {
        Self {
            buf: HashMap::new(),
        }
    }
    pub fn add_path<T: AsRef<Path>>(&mut self, path: T) {
        let path_ref = path.as_ref();

        let path = if let Some(str_path) = path_ref.to_str() {
            if let Some(stripped) = str_path.strip_prefix("~/") {
                if let Ok(home) = home_dir() {
                    home.join(stripped)
                } else {
                    return;
                }
            } else {
                path_ref.to_path_buf()
            }
        } else {
            path_ref.to_path_buf()
        };
        Self::scan_path(&path, &mut self.buf);
    }
    pub fn lookup_icon(&self, name: &str) -> Option<Option<Arc<Path>>> {
        self.buf.get(name).map(|p| p.clone())
    }
    fn scan_path(path: &Path, buf: &mut HashMap<String, Option<Arc<Path>>>) {
        // Early return if its not a scannable directory
        if !path.exists() || !path.is_dir() {
            return;
        }

        let Ok(entries) = std::fs::read_dir(path) else {
            return;
        };
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                Self::scan_path(&entry_path, buf);
            } else if let Some(ext) = entry_path.extension().and_then(|e| e.to_str()) {
                let is_icon = matches!(ext.to_ascii_lowercase().as_str(), "png" | "svg");
                if is_icon {
                    if let Some(stem) = entry_path.file_stem().and_then(|s| s.to_str()) {
                        let stem = stem.to_string();
                        let arc_path: Arc<Path> = Arc::from(entry_path.into_boxed_path());
                        buf.entry(stem).or_insert(Some(arc_path));
                    }
                }
            }
        }
    }
}

pub struct IconThemeGuard;
impl<'g> IconThemeGuard {
    fn get_theme() -> Result<&'g RwLock<CustomIconTheme>, SherlockError> {
        ICONS.get().ok_or_else(|| {
            sherlock_error!(
                SherlockErrorType::ConfigError(None),
                "Config not initialized".to_string()
            )
        })
    }

    fn get_read() -> Result<RwLockReadGuard<'g, CustomIconTheme>, SherlockError> {
        Self::get_theme()?.read().map_err(|_| {
            sherlock_error!(
                SherlockErrorType::ConfigError(None),
                "Failed to acquire write lock on config".to_string()
            )
        })
    }

    fn get_write() -> Result<RwLockWriteGuard<'g, CustomIconTheme>, SherlockError> {
        Self::get_theme()?.write().map_err(|_| {
            sherlock_error!(
                SherlockErrorType::ConfigError(None),
                "Failed to acquire write lock on config".to_string()
            )
        })
    }

    pub fn _read() -> Result<RwLockReadGuard<'g, CustomIconTheme>, SherlockError> {
        Self::get_read()
    }

    pub fn add_path<T: AsRef<Path>>(path: T) -> Result<(), SherlockError> {
        let mut inner = Self::get_write()?;
        inner.add_path(path);
        Ok(())
    }

    pub fn lookup_icon(name: &str) -> Result<Option<Option<Arc<Path>>>, SherlockError> {
        let inner = Self::get_read()?;
        Ok(inner.lookup_icon(name))
    }

    pub fn _write_key<F>(key_fn: F) -> Result<(), SherlockError>
    where
        F: FnOnce(&mut CustomIconTheme),
    {
        let mut config = Self::get_write()?;
        key_fn(&mut config);
        Ok(())
    }
}

pub fn resolve_icon_path(name: &str) -> Option<Arc<Path>> {
    // check if previously cached
    if let Ok(Some(icon)) = IconThemeGuard::lookup_icon(name) {
        return icon;
    }

    fn write_to_cache(name: &str, result: Option<Arc<Path>>) {
        if let Ok(mut cache) = IconThemeGuard::get_write() {
            cache.buf.insert(name.to_string(), result);
        }
    }

    // retrieve new
    let result: Option<Arc<Path>> = (|| {
        let icon_path = lookup_icon(name)
            .with_size(128)
            .with_search_paths(&["~/.local/share/icons/"])
            .ok()?
            .next()?
            .map(|i| i.path)
            .ok()?;

        Some(Arc::from(icon_path.into_boxed_path()))
    })();

    if result.is_some() {
        write_to_cache(name, result.clone());
        return result;
    }

    if let Some(result) = freedesktop_icons::lookup(name).with_size(128).find() {
        let result: Arc<Path> = Arc::from(result.into_boxed_path());
        write_to_cache(name, Some(result.clone()));
        return Some(result);
    }

    None
}
