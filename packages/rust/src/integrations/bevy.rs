use crate::types::JandiColor;

impl From<JandiColor> for bevy_color::Color {
    fn from(color: JandiColor) -> Self {
        bevy_color::Color::srgb_u8(color.rgb.r, color.rgb.g, color.rgb.b)
    }
}

impl From<&JandiColor> for bevy_color::Color {
    fn from(color: &JandiColor) -> Self {
        bevy_color::Color::srgb_u8(color.rgb.r, color.rgb.g, color.rgb.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::PRIMARY;
    #[test]
    fn test_bevy_conversion_owned() {
        let color: bevy_color::Color = PRIMARY.into();
        assert_eq!(color, bevy_color::Color::srgb_u8(61, 95, 128));
    }

    #[test]
    fn test_bevy_conversion_borrowed() {
        let color: bevy_color::Color = (&PRIMARY).into();
        assert_eq!(color, bevy_color::Color::srgb_u8(61, 95, 128));
    }
}
