use std::sync::Arc;

use gpui::{
    AnyElement, Image, ImageSource, IntoElement, ParentElement, Resource, Styled, div, img,
    linear_gradient, px, rgb,
};

use crate::{
    launcher::{ExecAttrs, weather_launcher::WeatherData},
    loader::utils::AppData,
    utils::errors::SherlockError,
};

macro_rules! renderable_enum {
    (
        enum $name:ident {
            $($variant:ident($inner:ty)),* $(,)?
        }
    ) => {
        #[derive(Clone)]
        pub enum $name {
            $($variant($inner)),*
        }

        impl RenderableChildImpl for $name {
            fn render(&self, icon: Option<Arc<std::path::Path>>, is_selected: bool) -> AnyElement {
                match self {
                    $(Self::$variant(inner) => inner.render(icon, is_selected)),*
                }
            }

            fn execute(&self, keyword: &str) -> Result<bool, SherlockError> {
                match self {
                    $(Self::$variant(inner) => inner.execute(keyword)),*
                }
            }

            fn priority(&self) -> f32 {
                match self {
                    $(Self::$variant(inner) => inner.priority()),*
                }
            }

            fn search(&self) -> String {
                match self {
                    $(Self::$variant(inner) => inner.search()),*
                }
            }

            fn icon(&self) -> Option<String> {
                match self {
                    $(Self::$variant(inner) => inner.icon()),*
                }
            }
        }
    };
}

renderable_enum! {
    enum RenderableChild {
        AppLike(AppData),
        WeatherLike(WeatherData),
    }
}

impl RenderableChild {
    pub fn get_exec(&self) -> Option<String> {
        match self {
            Self::AppLike(ad) => ad.get_exec(),
            _ => None,
        }
    }
}

pub trait RenderableChildImpl {
    fn render(&self, icon: Option<Arc<std::path::Path>>, is_selected: bool) -> AnyElement;
    fn execute(&self, keyword: &str) -> Result<bool, SherlockError>;
    fn priority(&self) -> f32;
    fn search(&self) -> String;
    fn icon(&self) -> Option<String>;
}

impl RenderableChildImpl for AppData {
    fn render(&self, icon: Option<Arc<std::path::Path>>, is_selected: bool) -> AnyElement {
        div()
            .px_4()
            .py_2()
            .w_full()
            .flex()
            .gap_5()
            .items_center()
            .child(if let Some(icon) = icon {
                img(ImageSource::Resource(Resource::Path(icon))).size(px(24.))
            } else {
                img(ImageSource::Image(Arc::new(Image::empty()))).size(px(24.))
            })
            .child(
                div()
                    .flex_col()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(if is_selected {
                                rgb(0xffffff)
                            } else {
                                rgb(0xcccccc)
                            })
                            .overflow_hidden()
                            .text_ellipsis()
                            .whitespace_nowrap()
                            .child(self.name.clone()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(if is_selected {
                                rgb(0x999999)
                            } else {
                                rgb(0x666666)
                            })
                            .children(
                                self.launcher
                                    .name
                                    .as_ref()
                                    .map(|name| div().child(name.clone())),
                            ),
                    ),
            )
            .into_any_element()
    }
    fn execute(&self, keyword: &str) -> Result<bool, SherlockError> {
        let attrs = ExecAttrs::from(self);
        self.launcher.execute(&attrs, keyword)
    }
    fn priority(&self) -> f32 {
        self.priority
    }
    fn search(&self) -> String {
        self.search_string.clone()
    }
    fn icon(&self) -> Option<String> {
        self.icon.clone()
    }
}

impl RenderableChildImpl for WeatherData {
    fn execute(&self, _keyword: &str) -> Result<bool, SherlockError> {
        Ok(false)
    }
    fn priority(&self) -> f32 {
        0.0
    }
    fn search(&self) -> String {
        String::new()
    }
    fn icon(&self) -> Option<String> {
        Some(self.icon.clone())
    }
    fn render(&self, icon: Option<Arc<std::path::Path>>, _is_selected: bool) -> AnyElement {
        div()
            .px_4()
            .py_2()
            .rounded_md()
            .bg({
                let (p1, p2) = self.css.background();
                linear_gradient(90., p1, p2)
            })
            .flex_col()
            .gap_5()
            .items_center()
            .text_size(px(12.0))
            .child(self.format_str.clone())
            .child(
                div()
                    .flex()
                    .gap_5()
                    .child(if let Some(icon) = icon {
                        img(ImageSource::Resource(Resource::Path(icon))).size(px(24.))
                    } else {
                        img(ImageSource::Image(Arc::new(Image::empty()))).size(px(24.))
                    })
                    .child(div().text_size(px(40.0)).child(self.temperature.clone())),
            )
            .into_any_element()
    }
}

pub trait SherlockSearch {
    /// Both self and substring should already be lowercased to increase performance
    fn fuzzy_match<'a>(&'a self, substring: &'a str) -> bool;
}

impl<T: AsRef<str>> SherlockSearch for T {
    fn fuzzy_match(&self, pattern: &str) -> bool {
        let t_bytes = self.as_ref().as_bytes();
        let p_bytes = pattern.as_bytes();

        // Early return for empty bytes
        if p_bytes.is_empty() {
            return true;
        }
        if t_bytes.is_empty() {
            return false;
        }

        let mut current_target = t_bytes;

        // memchr find first search byte
        while let Some(pos) = memchr::memchr(p_bytes[0], current_target) {
            if sequential_check(p_bytes, &current_target[pos..], 5) {
                return true;
            }
            // Move past the current match to find the next possible start
            if pos + 1 >= current_target.len() {
                break;
            }
            current_target = &current_target[pos + 1..];
        }

        false
    }
}

fn sequential_check(pattern: &[u8], target: &[u8], window_size: usize) -> bool {
    // pattern[0] was already matched by memchr at target[0]
    let mut t_idx = 1; 

    // We start from the second character (index 1)
    for &pattern_char in &pattern[1..] {
        // The window starts at t_idx and ends at t_idx + window_size
        let limit = std::cmp::min(t_idx + window_size, target.len());
        let mut found = false;

        while t_idx < limit {
            if target[t_idx] == pattern_char {
                t_idx += 1; // Start searching for the NEXT char from here
                found = true;
                break;
            }
            t_idx += 1;
        }

        // If the inner loop finishes without finding the char, the chain is broken
        if !found {
            return false;
        }
    }

    true
}
