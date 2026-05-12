use crate::types::JandiColor;

impl From<JandiColor> for ratatui::style::Color {
    fn from(color: JandiColor) -> Self {
        ratatui::style::Color::Rgb(color.rgb.r, color.rgb.g, color.rgb.b)
    }
}

impl From<&JandiColor> for ratatui::style::Color {
    fn from(color: &JandiColor) -> Self {
        ratatui::style::Color::Rgb(color.rgb.r, color.rgb.g, color.rgb.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::PRIMARY;
    #[test]
    fn test_ratatui_conversion_owned() {
        let color: ratatui::style::Color = PRIMARY.into();
        assert_eq!(color, ratatui::style::Color::Rgb(61, 95, 128));
    }

    #[test]
    fn test_ratatui_conversion_borrowed() {
        let color: ratatui::style::Color = (&PRIMARY).into();
        assert_eq!(color, ratatui::style::Color::Rgb(61, 95, 128));
    }
}
