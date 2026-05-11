//! Integration implementations for UI frameworks and serialization.
//!
//! Each submodule is feature-gated behind its corresponding feature flag.

/// `From<JandiColor>` and `From<&JandiColor>` for `bevy_color::Color`.
#[cfg(feature = "bevy")]
pub mod bevy;
/// `From<JandiColor>` and `From<&JandiColor>` for `crossterm::style::Color`.
#[cfg(feature = "crossterm")]
pub mod crossterm;
/// `From<JandiColor>` and `From<&JandiColor>` for `egui::Color32`.
#[cfg(feature = "egui")]
pub mod egui;
/// `From<JandiColor>` and `From<&JandiColor>` for `iced_core::Color`.
#[cfg(feature = "iced")]
pub mod iced;
/// `From<JandiColor>` and `From<&JandiColor>` for `palette::Srgb<u8>`.
#[cfg(feature = "palette")]
pub mod palette_crate;
/// `From<JandiColor>` and `From<&JandiColor>` for `ratatui::style::Color`.
#[cfg(feature = "ratatui")]
pub mod ratatui;
/// Serde `Serialize` and `Deserialize` implementations for `JandiColor` and
/// `OxidationStage`.
#[cfg(feature = "serde")]
pub mod serde_impl;
