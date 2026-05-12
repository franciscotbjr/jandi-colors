use crate::types::JandiColor;

impl From<JandiColor> for palette::Srgb<u8> {
    fn from(color: JandiColor) -> Self {
        palette::Srgb::new(color.rgb.r, color.rgb.g, color.rgb.b)
    }
}

impl From<&JandiColor> for palette::Srgb<u8> {
    fn from(color: &JandiColor) -> Self {
        palette::Srgb::new(color.rgb.r, color.rgb.g, color.rgb.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::{PRIMARY, SUCO_VERDE, TINTA_GUERRA};
    #[test]
    fn test_palette_conversion_owned() {
        assert_eq!(
            palette::Srgb::<u8>::from(PRIMARY),
            palette::Srgb::new(61, 95, 128)
        );
        assert_eq!(
            palette::Srgb::<u8>::from(SUCO_VERDE),
            palette::Srgb::new(200, 213, 204)
        );
        assert_eq!(
            palette::Srgb::<u8>::from(TINTA_GUERRA),
            palette::Srgb::new(10, 20, 36)
        );
    }

    #[test]
    fn test_palette_conversion_borrowed() {
        assert_eq!(
            palette::Srgb::<u8>::from(&PRIMARY),
            palette::Srgb::new(61, 95, 128)
        );
        assert_eq!(
            palette::Srgb::<u8>::from(&SUCO_VERDE),
            palette::Srgb::new(200, 213, 204)
        );
        assert_eq!(
            palette::Srgb::<u8>::from(&TINTA_GUERRA),
            palette::Srgb::new(10, 20, 36)
        );
    }
}
