//! Common utilities for verandah widget plugins.
//!
//! Badge composition, colour parsing, font loading, image effects and text
//! rendering live in [`verandah_image`]; this crate re-exports them unchanged so
//! existing plugins keep compiling, and adds the serde helpers that are specific
//! to plugin configuration.
//!
//! # Example
//!
//! ```ignore
//! use verandah_plugin_utils::prelude::*;
//!
//! // Parse colors from config
//! let colors = parse_colors(&config.colors);
//! let fg = get_color(&colors, "fg", Rgba([255, 255, 255, 255]));
//!
//! // Draw centered text
//! let mut img = RgbaImage::new(72, 72);
//! draw_centered_text(&mut img, "Hello", fg, 0.1);
//!
//! // Badge the frame with a runtime-computed mark
//! apply_badge(&mut img, &BadgeSpec::new(Mark::Text(unread.to_string())));
//! ```

pub use verandah_image::{badge, colors, font, image, text};

pub mod serde;

/// Prelude module for convenient imports.
///
/// ```ignore
/// use verandah_plugin_utils::prelude::*;
/// ```
pub mod prelude {
    pub use verandah_image::prelude::*;

    // Serde utilities
    pub use crate::serde::IgnoredValue;
}
