#![no_std]
#![deny(missing_docs)]

//! # jandi-colors
//!
//! Color palette derived from the blue pigment of jenipapo (Genipa americana),
//! the ancestral dye of Brazil's indigenous peoples.
//!
//! ## Basic usage (zero dependencies)
//!
//! ```
//! use jandi_colors::{PRIMARY, PALETTE, by_slug};
//!
//! assert_eq!(PRIMARY.hex, "#3D5F80");
//! assert_eq!(PALETTE.len(), 8);
//!
//! let color = by_slug("jandi-suco-verde").expect("slug exists");
//! assert_eq!(color.name, "Suco verde");
//! ```
//!
//! ## Feature flags
//!
//! Enable integrations with UI frameworks via `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! jandi-colors = { version = "0.1", features = ["ratatui", "serde", "contrast"] }
//! ```

#[cfg(feature = "std")]
extern crate std;

/// Core types for the Jandí palette: `JandiColor`, `Rgb`, `Hsl`, `OxidationStage`.
pub mod types;

/// All 8 color constants, `PALETTE` array, `by_slug()` lookup, and the `JANDI` alias.
pub mod palette;

/// WCAG 2.1 contrast utilities (luminance, ratio, AA/AAA predicates).
#[cfg(feature = "contrast")]
pub mod contrast;

/// Feature-gated integrations with UI frameworks and serialization.
#[cfg(any(
    feature = "ratatui",
    feature = "iced",
    feature = "bevy",
    feature = "egui",
    feature = "crossterm",
    feature = "palette",
    feature = "serde"
))]
pub mod integrations;

pub use palette::*;
pub use types::*;

#[cfg(feature = "contrast")]
pub use contrast::*;

#[cfg(test)]
mod tests {
    use crate::{JANDI, PALETTE, PRIMARY, VERSION, by_slug};

    #[test]
    fn test_re_exports_accessible() {
        assert_eq!(PRIMARY.hex, "#3D5F80");
        assert_eq!(PALETTE.len(), 8);
        assert_eq!(VERSION, "0.1.0");
        assert_eq!(JANDI.hex, PRIMARY.hex);
        assert!(by_slug("jandi-oby").is_some());
    }
}
