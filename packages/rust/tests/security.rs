use jandi_colors::*;

#[test]
fn by_slug_handles_very_long_input() {
    let long = "x".repeat(10_000);
    assert!(by_slug(&long).is_none());
}

#[test]
fn by_slug_handles_empty_input() {
    assert!(by_slug("").is_none());
}

#[test]
fn by_slug_handles_unicode_input() {
    assert!(by_slug("jand\u{00ed}-primary").is_none());
    assert!(by_slug("\u{1f3a8}").is_none());
}

#[cfg(feature = "contrast")]
mod contrast_adversarial {
    use super::*;

    #[test]
    fn get_contrast_pair_negative_threshold_returns_all_pairs() {
        let pairs = get_contrast_pair(&PALETTE, -5.0);
        assert_eq!(pairs.len(), 28);
    }

    #[test]
    fn get_contrast_pair_nan_threshold_returns_no_pairs() {
        let pairs = get_contrast_pair(&PALETTE, f32::NAN);
        assert_eq!(pairs.len(), 0);
    }

    #[test]
    fn get_contrast_pair_infinity_threshold_returns_no_pairs() {
        let pairs = get_contrast_pair(&PALETTE, f32::INFINITY);
        assert_eq!(pairs.len(), 0);
    }
}

#[cfg(feature = "serde")]
mod serde_adversarial {
    use super::*;

    #[test]
    fn deserialize_unknown_slug_returns_err_without_reflecting_payload() {
        let payload = r#"{"slug": "evil<svg onload=alert(1)>"}"#;
        let result: Result<JandiColor, _> = serde_json::from_str(payload);
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(
            !msg.contains("<svg"),
            "error message reflects payload: {msg}"
        );
        assert!(
            !msg.contains("alert"),
            "error message reflects payload: {msg}"
        );
    }

    #[test]
    fn deserialize_missing_slug_returns_err() {
        let payload = r#"{}"#;
        let result: Result<JandiColor, _> = serde_json::from_str(payload);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_wrong_slug_type_returns_err() {
        let payload = r#"{"slug": 123}"#;
        let result: Result<JandiColor, _> = serde_json::from_str(payload);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_malformed_json_returns_err() {
        let payload = r#"{slug: "jandi-primary""#;
        let result: Result<JandiColor, _> = serde_json::from_str(payload);
        assert!(result.is_err());
    }
}
