use regex::Regex;

const PALETTE_TS: &str = include_str!("../../typescript/src/palette.ts");

#[derive(Debug)]
struct TsColor {
    slug: String,
    name: String,
    hex: String,
    r: u8,
    g: u8,
    b: u8,
    h: u16,
    s: u8,
    l: u8,
    stage: String,
    description: String,
}

fn parse_ts_palette(source: &str) -> Vec<TsColor> {
    let mut colors = Vec::new();
    let blocks: Vec<&str> = source
        .split("export const")
        .skip(1)
        .filter(|b| b.contains("JandiColor"))
        .collect();

    let re_str = Regex::new(r#""([^"]+)""#).unwrap();
    let re_rgb = Regex::new(r"r:\s*(\d+),\s*g:\s*(\d+),\s*b:\s*(\d+)").unwrap();
    let re_hsl = Regex::new(r"h:\s*(\d+),\s*s:\s*(\d+),\s*l:\s*(\d+)").unwrap();

    for block in blocks {
        let values: Vec<&str> = re_str
            .captures_iter(block)
            .filter_map(|c| c.get(1))
            .map(|m| m.as_str())
            .collect();

        let (name, slug, hex, stage, description) = if values.len() >= 5 {
            (
                values[0].to_string(),
                values[1].to_string(),
                values[2].to_string(),
                values[3].to_string(),
                values[4].to_string(),
            )
        } else {
            continue;
        };

        let rgb = re_rgb.captures(block).map(|c| {
            let r = c.get(1).unwrap().as_str().parse::<u8>().unwrap();
            let g = c.get(2).unwrap().as_str().parse::<u8>().unwrap();
            let b = c.get(3).unwrap().as_str().parse::<u8>().unwrap();
            (r, g, b)
        });

        let hsl = re_hsl.captures(block).map(|c| {
            let h = c.get(1).unwrap().as_str().parse::<u16>().unwrap();
            let s = c.get(2).unwrap().as_str().parse::<u8>().unwrap();
            let l = c.get(3).unwrap().as_str().parse::<u8>().unwrap();
            (h, s, l)
        });

        if let (Some((r, g, b)), Some((h, s, l))) = (rgb, hsl) {
            colors.push(TsColor {
                slug,
                name,
                hex,
                r,
                g,
                b,
                h,
                s,
                l,
                stage,
                description,
            });
        }
    }

    colors
}

#[test]
fn test_cross_format_parses_8_colors() {
    let ts_colors = parse_ts_palette(PALETTE_TS);
    assert_eq!(
        ts_colors.len(),
        8,
        "expected 8 colors from palette.ts, got {}",
        ts_colors.len()
    );
}

#[test]
fn test_all_colors_match_typescript_source_of_truth() {
    let ts_colors = parse_ts_palette(PALETTE_TS);
    let palette = &jandi_colors::PALETTE;

    assert_eq!(ts_colors.len(), palette.len(), "palette size mismatch");

    for (i, ts) in ts_colors.iter().enumerate() {
        let rs = &palette[i];

        assert_eq!(
            rs.slug, ts.slug,
            "[{}] slug mismatch: Rust '{}' vs TS '{}'",
            i, rs.slug, ts.slug
        );
        assert_eq!(
            rs.name, ts.name,
            "[{}] name mismatch for '{}': Rust '{}' vs TS '{}'",
            i, ts.slug, rs.name, ts.name
        );
        assert_eq!(
            rs.hex, ts.hex,
            "[{}] hex mismatch for '{}': Rust '{}' vs TS '{}'",
            i, ts.slug, rs.hex, ts.hex
        );
        assert_eq!(
            rs.rgb.r, ts.r,
            "[{}] rgb.r mismatch for '{}': Rust {} vs TS {}",
            i, ts.slug, rs.rgb.r, ts.r
        );
        assert_eq!(
            rs.rgb.g, ts.g,
            "[{}] rgb.g mismatch for '{}': Rust {} vs TS {}",
            i, ts.slug, rs.rgb.g, ts.g
        );
        assert_eq!(
            rs.rgb.b, ts.b,
            "[{}] rgb.b mismatch for '{}': Rust {} vs TS {}",
            i, ts.slug, rs.rgb.b, ts.b
        );
        assert_eq!(
            rs.hsl.h, ts.h,
            "[{}] hsl.h mismatch for '{}': Rust {} vs TS {}",
            i, ts.slug, rs.hsl.h, ts.h
        );
        assert_eq!(
            rs.hsl.s, ts.s,
            "[{}] hsl.s mismatch for '{}': Rust {} vs TS {}",
            i, ts.slug, rs.hsl.s, ts.s
        );
        assert_eq!(
            rs.hsl.l, ts.l,
            "[{}] hsl.l mismatch for '{}': Rust {} vs TS {}",
            i, ts.slug, rs.hsl.l, ts.l
        );
        assert_eq!(
            rs.oxidation_stage.to_string(),
            ts.stage,
            "[{}] oxidation_stage mismatch for '{}': Rust '{}' vs TS '{}'",
            i,
            ts.slug,
            rs.oxidation_stage,
            ts.stage
        );
        assert_eq!(
            rs.description, ts.description,
            "[{}] description mismatch for '{}'",
            i, ts.slug
        );
    }
}
