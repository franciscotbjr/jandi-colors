use jandi_colors::*;

fn parse_hex(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    (r, g, b)
}

fn hex_to_hsl(hex: &str) -> (u16, u8, u8) {
    let (r, g, b) = parse_hex(hex);
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;
    let max = rf.max(gf.max(bf));
    let min = rf.min(gf.min(bf));
    let l = (max + min) / 2.0;
    if max == min {
        return (0, 0, (l * 100.0).round() as u8);
    }
    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };
    let h = if max == rf {
        ((gf - bf) / d + if gf < bf { 6.0 } else { 0.0 }) / 6.0
    } else if max == gf {
        ((bf - rf) / d + 2.0) / 6.0
    } else {
        ((rf - gf) / d + 4.0) / 6.0
    };
    (
        (h * 360.0).round() as u16,
        (s * 100.0).round() as u8,
        (l * 100.0).round() as u8,
    )
}

#[test]
fn test_all_hex_unique() {
    let mut hexes: [&str; 8] = [""; 8];
    for (i, color) in PALETTE.iter().enumerate() {
        hexes[i] = color.hex;
    }
    for i in 0..hexes.len() {
        for j in (i + 1)..hexes.len() {
            assert_ne!(hexes[i], hexes[j], "duplicate hex: {}", hexes[i]);
        }
    }
}

#[test]
fn test_all_slugs_unique() {
    let mut slugs: [&str; 8] = [""; 8];
    for (i, color) in PALETTE.iter().enumerate() {
        slugs[i] = color.slug;
    }
    for i in 0..slugs.len() {
        for j in (i + 1)..slugs.len() {
            assert_ne!(slugs[i], slugs[j], "duplicate slug: {}", slugs[i]);
        }
    }
}

#[test]
fn test_all_names_unique() {
    let mut names: [&str; 8] = [""; 8];
    for (i, color) in PALETTE.iter().enumerate() {
        names[i] = color.name;
    }
    for i in 0..names.len() {
        for j in (i + 1)..names.len() {
            assert_ne!(names[i], names[j], "duplicate name: {}", names[i]);
        }
    }
}

#[test]
fn test_hex_rgb_consistency() {
    for color in PALETTE.iter() {
        let (r, g, b) = parse_hex(color.hex);
        assert_eq!(r, color.rgb.r, "hex RGB mismatch for {}", color.slug);
        assert_eq!(g, color.rgb.g, "hex RGB mismatch for {}", color.slug);
        assert_eq!(b, color.rgb.b, "hex RGB mismatch for {}", color.slug);
    }
}

#[test]
fn test_rgb_hsl_consistency() {
    for color in PALETTE.iter() {
        let (h, s, l) = hex_to_hsl(color.hex);
        let tolerance_h = 2;
        let tolerance_sl = 2;
        let h_diff = if h > color.hsl.h {
            h - color.hsl.h
        } else {
            color.hsl.h - h
        };
        let s_diff = if s > color.hsl.s {
            s - color.hsl.s
        } else {
            color.hsl.s - s
        };
        let l_diff = if l > color.hsl.l {
            l - color.hsl.l
        } else {
            color.hsl.l - l
        };
        assert!(
            h_diff <= tolerance_h,
            "HSL hue mismatch for {}: expected {} got {}",
            color.slug,
            color.hsl.h,
            h
        );
        assert!(
            s_diff <= tolerance_sl,
            "HSL saturation mismatch for {}: expected {} got {}",
            color.slug,
            color.hsl.s,
            s
        );
        assert!(
            l_diff <= tolerance_sl,
            "HSL lightness mismatch for {}: expected {} got {}",
            color.slug,
            color.hsl.l,
            l
        );
    }
}

#[test]
fn test_luminosity_descending() {
    for i in 0..(PALETTE.len() - 1) {
        assert!(
            PALETTE[i].hsl.l >= PALETTE[i + 1].hsl.l,
            "PALETTE not sorted by luminosity descending at index {}: {} (l={}) vs {} (l={})",
            i,
            PALETTE[i].slug,
            PALETTE[i].hsl.l,
            PALETTE[i + 1].slug,
            PALETTE[i + 1].hsl.l,
        );
    }
}

#[test]
fn test_jandi_alias() {
    assert_eq!(JANDI.hex, PRIMARY.hex);
    assert_eq!(JANDI.rgb.r, PRIMARY.rgb.r);
    assert_eq!(JANDI.rgb.g, PRIMARY.rgb.g);
    assert_eq!(JANDI.rgb.b, PRIMARY.rgb.b);
    assert_eq!(JANDI.slug, PRIMARY.slug);
    assert_eq!(JANDI.name, PRIMARY.name);
}

#[test]
fn test_version_consistency() {
    assert_eq!(VERSION, "0.1.0");
    assert_eq!(PALETTE.len(), 8);
}
