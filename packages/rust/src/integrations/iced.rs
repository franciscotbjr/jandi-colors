use crate::types::JandiColor;

impl From<JandiColor> for iced_core::Color {
    fn from(color: JandiColor) -> Self {
        iced_core::Color::from_rgb8(color.rgb.r, color.rgb.g, color.rgb.b)
    }
}

impl From<&JandiColor> for iced_core::Color {
    fn from(color: &JandiColor) -> Self {
        iced_core::Color::from_rgb8(color.rgb.r, color.rgb.g, color.rgb.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::PRIMARY;
    #[test]
    fn test_iced_conversion_owned() {
        let color: iced_core::Color = PRIMARY.into();
        assert_eq!(color, iced_core::Color::from_rgb8(61, 95, 128));
    }

    #[test]
    fn test_iced_conversion_borrowed() {
        let color: iced_core::Color = (&PRIMARY).into();
        assert_eq!(color, iced_core::Color::from_rgb8(61, 95, 128));
    }
}
