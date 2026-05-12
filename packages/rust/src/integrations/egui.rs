use crate::types::JandiColor;

impl From<JandiColor> for egui::Color32 {
    fn from(color: JandiColor) -> Self {
        egui::Color32::from_rgb(color.rgb.r, color.rgb.g, color.rgb.b)
    }
}

impl From<&JandiColor> for egui::Color32 {
    fn from(color: &JandiColor) -> Self {
        egui::Color32::from_rgb(color.rgb.r, color.rgb.g, color.rgb.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::PRIMARY;
    #[test]
    fn test_egui_conversion_owned() {
        let color: egui::Color32 = PRIMARY.into();
        assert_eq!(color, egui::Color32::from_rgb(61, 95, 128));
    }

    #[test]
    fn test_egui_conversion_borrowed() {
        let color: egui::Color32 = (&PRIMARY).into();
        assert_eq!(color, egui::Color32::from_rgb(61, 95, 128));
    }
}
