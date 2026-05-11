use jandi_colors::*;

#[cfg(feature = "ratatui")]
#[test]
fn test_ratatui_conversion() {
    let _: ratatui::style::Color = PRIMARY.into();
    let _: ratatui::style::Color = SUCO_VERDE.into();
    let _: ratatui::style::Color = TINTA_GUERRA.into();
}

#[cfg(feature = "iced")]
#[test]
fn test_iced_conversion() {
    let _: iced_core::Color = PRIMARY.into();
    let _: iced_core::Color = SUCO_VERDE.into();
    let _: iced_core::Color = TINTA_GUERRA.into();
}

#[cfg(feature = "bevy")]
#[test]
fn test_bevy_conversion() {
    let _: bevy_color::Color = PRIMARY.into();
    let _: bevy_color::Color = SUCO_VERDE.into();
    let _: bevy_color::Color = TINTA_GUERRA.into();
}

#[cfg(feature = "egui")]
#[test]
fn test_egui_conversion() {
    let _: egui::Color32 = PRIMARY.into();
    let _: egui::Color32 = SUCO_VERDE.into();
    let _: egui::Color32 = TINTA_GUERRA.into();
}

#[cfg(feature = "crossterm")]
#[test]
fn test_crossterm_conversion() {
    let _: crossterm::style::Color = PRIMARY.into();
    let _: crossterm::style::Color = SUCO_VERDE.into();
    let _: crossterm::style::Color = TINTA_GUERRA.into();
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip() {
    let json = serde_json::to_string(&PRIMARY).expect("serialize");
    let deserialized: JandiColor = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(PRIMARY.name, deserialized.name);
    assert_eq!(PRIMARY.slug, deserialized.slug);
    assert_eq!(PRIMARY.hex, deserialized.hex);
    assert_eq!(PRIMARY.rgb, deserialized.rgb);
    assert_eq!(PRIMARY.hsl, deserialized.hsl);
    assert_eq!(PRIMARY.oxidation_stage, deserialized.oxidation_stage);
    assert_eq!(PRIMARY.description, deserialized.description);

    let json = serde_json::to_string(&SUCO_VERDE).expect("serialize");
    let deserialized: JandiColor = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(SUCO_VERDE.hex, deserialized.hex);

    let json = serde_json::to_string(&TINTA_GUERRA).expect("serialize");
    let deserialized: JandiColor = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(TINTA_GUERRA.hex, deserialized.hex);
}
