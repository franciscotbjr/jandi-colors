//! WCAG 2.1 contrast utilities for the Jandí palette.
//!
//! Feature-gated behind `"contrast"`. Provides relative luminance calculation,
//! contrast ratios, WCAG level predicates, and a pair-finding utility.
//!
//! ## Example
//!
//! ```
//! use jandi_colors::{SUCO_VERDE, YANDI, wcag_aa, wcag_aaa, contrast_ratio};
//!
//! let ratio = contrast_ratio(&SUCO_VERDE, &YANDI);
//! assert!(ratio >= 10.0);
//! assert!(wcag_aaa(&SUCO_VERDE, &YANDI));
//! assert!(wcag_aa(&SUCO_VERDE, &YANDI));
//! ```

use crate::types::JandiColor;
use arrayvec::ArrayVec;
use libm::powf;

const WCAG_AA_LARGE: f32 = 3.0;
const WCAG_AA: f32 = 4.5;
const WCAG_AAA: f32 = 7.0;

/// C(8, 2) — maximum number of unordered color pairs from the 8-color palette.
const MAX_PAIRS: usize = 28;

fn srgb_to_linear(c: u8) -> f32 {
    let c = c as f32 / 255.0;
    if c <= 0.03928 {
        c / 12.92
    } else {
        powf((c + 0.055) / 1.055, 2.4)
    }
}

/// Relative luminance per WCAG 2.1 definition.
pub fn relative_luminance(color: &JandiColor) -> f32 {
    let r = srgb_to_linear(color.rgb.r);
    let g = srgb_to_linear(color.rgb.g);
    let b = srgb_to_linear(color.rgb.b);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Contrast ratio between two colors per WCAG 2.1.
pub fn contrast_ratio(a: &JandiColor, b: &JandiColor) -> f32 {
    let l1 = relative_luminance(a);
    let l2 = relative_luminance(b);
    let lighter = if l1 > l2 { l1 } else { l2 };
    let darker = if l1 > l2 { l2 } else { l1 };
    (lighter + 0.05) / (darker + 0.05)
}

/// Whether the contrast ratio meets WCAG AA for large text (>= 3.0:1).
pub fn wcag_aa_large(a: &JandiColor, b: &JandiColor) -> bool {
    contrast_ratio(a, b) >= WCAG_AA_LARGE
}

/// Whether the contrast ratio meets WCAG AA for normal text (>= 4.5:1).
pub fn wcag_aa(a: &JandiColor, b: &JandiColor) -> bool {
    contrast_ratio(a, b) >= WCAG_AA
}

/// Whether the contrast ratio meets WCAG AAA (>= 7.0:1).
pub fn wcag_aaa(a: &JandiColor, b: &JandiColor) -> bool {
    contrast_ratio(a, b) >= WCAG_AAA
}

/// Whether a color is considered "light" (lightness > 50).
pub fn is_light(color: &JandiColor) -> bool {
    color.hsl.l > 50
}

/// Find all unordered color pairs with contrast ratio >= threshold.
///
/// Contrast ratio is symmetric — only one direction of each pair is returned.
/// With 8 colors, at most C(8, 2) = 28 pairs are possible.
pub fn get_contrast_pair(
    palette: &[JandiColor; 8],
    threshold: f32,
) -> ArrayVec<(&JandiColor, &JandiColor), MAX_PAIRS> {
    let mut result: ArrayVec<(&JandiColor, &JandiColor), MAX_PAIRS> = ArrayVec::new();
    for (i, fg) in palette.iter().enumerate() {
        for bg in palette.iter().skip(i + 1) {
            if contrast_ratio(fg, bg) >= threshold {
                result.push((fg, bg));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_light_true() {
        assert!(is_light(&crate::SUCO_VERDE));
        assert!(is_light(&crate::BRISA));
    }

    #[test]
    fn test_is_light_false() {
        assert!(!is_light(&crate::PRIMARY));
        assert!(!is_light(&crate::GENIPINA));
    }

    #[test]
    fn test_wcag_aa_pass() {
        assert!(wcag_aa(&crate::SUCO_VERDE, &crate::GENIPINA));
        assert!(wcag_aa(&crate::SUCO_VERDE, &crate::YANDI));
    }

    #[test]
    fn test_wcag_aa_fail() {
        assert!(!wcag_aa(&crate::OBY, &crate::SUCO_VERDE));
    }
}
