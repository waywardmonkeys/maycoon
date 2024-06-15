use std::sync::Arc;

use dashmap::DashMap;
use peniko::{Blob, Font};

/// The font context. Contains font data.
#[derive(Clone, Debug)]
pub struct FontContext {
    default: Font,
    fonts: DashMap<String, Font>,
}

impl FontContext {
    /// Insert a font with a custom name.
    pub fn insert(&mut self, name: impl ToString, font: Font) {
        self.fonts.insert(name.to_string(), font);
    }

    /// Get a font by a specified name. Returns [None] if the font could not be found.
    pub fn get(&self, name: impl ToString) -> Option<Font> {
        self.fonts.get(&name.to_string()).map(|el| el.clone())
    }

    /// Removes a font. Returns [None] if the font does not exist.
    pub fn remove(&mut self, name: impl ToString) -> Option<()> {
        self.fonts.remove(&name.to_string()).map(|_| ())
    }

    /// Returns the default font (usually [Roboto](https://fonts.google.com/specimen/Roboto)).
    pub fn default_font(&self) -> &Font {
        &self.default
    }
}

impl Default for FontContext {
    fn default() -> Self {
        Self {
            default: Font::new(
                Blob::new(Arc::new(include_bytes!(
                    "../../../assets/data/Roboto-Regular.ttf"
                ))),
                0,
            ),
            fonts: DashMap::new(),
        }
    }
}
