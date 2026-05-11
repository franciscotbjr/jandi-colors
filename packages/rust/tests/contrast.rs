#![cfg(feature = "contrast")]

use jandi_colors::*;

#[test]
fn test_suco_verde_yandi_aaa() {
    assert!(wcag_aaa(&SUCO_VERDE, &YANDI));
}

#[test]
fn test_suco_verde_nhandi_aaa() {
    assert!(wcag_aaa(&SUCO_VERDE, &NHANDI));
}

#[test]
fn test_suco_verde_genipina_aa() {
    assert!(wcag_aa(&SUCO_VERDE, &GENIPINA));
}

#[test]
fn test_brisa_tinta_guerra_aaa() {
    assert!(wcag_aaa(&BRISA, &TINTA_GUERRA));
}

#[test]
fn test_brisa_yandi_aa() {
    assert!(wcag_aa(&BRISA, &YANDI));
}

#[test]
fn test_oby_suco_verde_does_not_meet_aa() {
    assert!(!wcag_aa(&OBY, &SUCO_VERDE));
}

#[test]
fn test_primary_suco_verde_aa_large() {
    assert!(wcag_aa_large(&PRIMARY, &SUCO_VERDE));
}

#[test]
fn test_tinta_guerra_suco_verde_aaa() {
    assert!(wcag_aaa(&TINTA_GUERRA, &SUCO_VERDE));
}

#[test]
fn test_tinta_guerra_brisa_aaa() {
    assert!(wcag_aaa(&TINTA_GUERRA, &BRISA));
}

#[test]
fn test_tinta_guerra_oby_aa_large() {
    assert!(wcag_aa_large(&TINTA_GUERRA, &OBY));
}

#[test]
fn test_is_light() {
    assert!(is_light(&SUCO_VERDE));
    assert!(is_light(&BRISA));
    assert!(!is_light(&OBY));
    assert!(!is_light(&PRIMARY));
    assert!(!is_light(&GENIPINA));
    assert!(!is_light(&NHANDI));
    assert!(!is_light(&YANDI));
    assert!(!is_light(&TINTA_GUERRA));
}

#[test]
fn test_get_contrast_pair() {
    let pairs = get_contrast_pair(&PALETTE, 4.5);
    assert!(
        !pairs.is_empty(),
        "should find at least one pair with ratio >= 4.5"
    );
}

#[test]
fn test_get_contrast_pair_zero_threshold_no_panic() {
    let pairs = get_contrast_pair(&PALETTE, 0.0);
    assert_eq!(pairs.len(), 28, "C(8,2) = 28 unique unordered pairs");
    for (i, (a, b)) in pairs.iter().enumerate() {
        assert!(!core::ptr::eq(*a, *b), "self-pair at index {i}");
    }
}
