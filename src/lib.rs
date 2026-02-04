//! Common utilities for verandah widget plugins.
//!
//! This crate provides shared functionality for building verandah plugins,
//! including:
//!
//! - **colors**: CSS color parsing (named colors and hex formats)
//! - **font**: System font loading via fontconfig
//! - **text**: Text measurement and rendering utilities
//! - **image**: Image effects (brightness pulse) and format conversions
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
//! // Apply brightness pulse for animation
//! apply_brightness_pulse(&mut img);
//! ```

pub mod colors;
pub mod font;
pub mod image;
pub mod text;

/// Prelude module for convenient imports.
///
/// Import everything commonly needed with:
/// ```ignore
/// use verandah_plugin_utils::prelude::*;
/// ```
pub mod prelude {
    // Re-export image types that plugins commonly use
    pub use ::image::{Rgb, RgbImage, Rgba, RgbaImage};

    // Colors
    pub use crate::colors::{get_color, hex as rgb, lookup as lookup_color, parse_colors};

    // Font
    pub use crate::font::get_system_monospace_font;

    // Text
    pub use crate::text::{
        draw_centered_text, draw_centered_text_with_reserved, find_optimal_scale,
        measure_text_width,
    };

    // Image utilities
    pub use crate::image::{
        apply_brightness_pulse, bytes_to_rgb, bytes_to_rgba, rgb_to_rgba, rgba_to_rgb, scale_image,
        to_greyscale,
    };
}
