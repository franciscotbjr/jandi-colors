# jandi-colors

**A color palette derived from the blue pigment of jenipapo (*Genipa americana*), the ancestral dye of Brazil's indigenous peoples.**

[![crates.io](https://img.shields.io/crates/v/jandi-colors.svg)](https://crates.io/crates/jandi-colors)
[![docs.rs](https://img.shields.io/docsrs/jandi-colors)](https://docs.rs/jandi-colors)
[![MSRV](https://img.shields.io/badge/msrv-1.95.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()

---

## Installation

```bash
cargo add jandi-colors
```

## The Palette

Eight tones derived from the real behavior of genipin pigment, from fresh green pulp to concentrated ritual war paint.

| Name | Hex | RGB | HSL | Oxidation Stage |
|------|-----|-----|-----|-----------------|
| **Suco verde** | `#C8D5CC` | 200, 213, 204 | 138°, 13%, 81% | Pre |
| **Brisa** | `#8FA7B3` | 143, 167, 179 | 200°, 19%, 63% | Initial |
| **Oby** | `#5D7F96` | 93, 127, 150 | 204°, 23%, 48% | Partial |
| **Jandí** | `#3D5F80` | 61, 95, 128 | 210°, 35%, 37% | Full |
| **Genipina** | `#2C4A6E` | 44, 74, 110 | 213°, 43%, 30% | Saturated |
| **Nhandí** | `#1E3452` | 30, 52, 82 | 215°, 46%, 22% | Deep |
| **Yandí** | `#11203A` | 17, 32, 58 | 218°, 55%, 15% | Concentrated |
| **Tinta de guerra** | `#0A1424` | 10, 20, 36 | 217°, 57%, 9% | Max |

## Basic API (zero dependencies)

```rust
use jandi_colors::*;

// Access any color by name constant
println!("Primary color: {}", PRIMARY);        // "Jandí (#3D5F80)"
println!("Hex: {}", PRIMARY.hex);              // "#3D5F80"
println!("RGB: {}, {}, {}", PRIMARY.rgb.r, PRIMARY.rgb.g, PRIMARY.rgb.b);
println!("HSL: {}°, {}%, {}%", PRIMARY.hsl.h, PRIMARY.hsl.s, PRIMARY.hsl.l);

// Iterate the full palette
for color in PALETTE {
    println!("{} - {}", color.slug, color.hex);
}

// Look up by slug
if let Some(color) = by_slug("jandi-oby") {
    println!("Found: {}", color.name);
}

// JANDI is an alias for PRIMARY
assert_eq!(JANDI.hex, PRIMARY.hex);

// Oxidation stage enum
use jandi_colors::OxidationStage;
assert_eq!(PRIMARY.oxidation_stage, OxidationStage::Full);
```

## Feature Flags

### UI Frameworks

Enable framework integrations via `From` trait implementations:

```toml
[dependencies]
jandi-colors = { version = "0.1", features = ["ratatui"] }
```

```rust
use jandi_colors::PRIMARY;

// ratatui
let color: ratatui::style::Color = PRIMARY.into();

// iced
let color: iced_core::Color = PRIMARY.into();

// bevy
let color: bevy_color::Color = PRIMARY.into();

// egui
let color: egui::Color32 = PRIMARY.into();

// crossterm
let color: crossterm::style::Color = PRIMARY.into();

// palette crate (color space conversions)
let color: palette::Srgb<u8> = PRIMARY.into();
```

### Serialization (serde)

```toml
[dependencies]
jandi-colors = { version = "0.1", features = ["serde"] }
```

```rust
use jandi_colors::PRIMARY;

let json = serde_json::to_string(&PRIMARY)?;
let color: jandi_colors::JandiColor = serde_json::from_str(&json)?;
```

### WCAG Contrast Utilities

```toml
[dependencies]
jandi-colors = { version = "0.1", features = ["contrast"] }
```

```rust
use jandi_colors::*;

let ratio = contrast_ratio(&SUCO_VERDE, &YANDI);
assert!(ratio >= 10.0);

assert!(wcag_aaa(&SUCO_VERDE, &YANDI));   // AAA: ratio >= 7.0
assert!(wcag_aa(&SUCO_VERDE, &YANDI));    // AA: ratio >= 4.5

assert!(is_light(&SUCO_VERDE));  // lightness > 50
assert!(!is_light(&PRIMARY));

// Find all accessible pairs
let pairs = get_contrast_pair(&PALETTE, 4.5);
for (fg, bg) in &pairs {
    println!("{} on {}: {:.1}:1", fg.name, bg.name, contrast_ratio(fg, bg));
}
```

## Accessibility

WCAG 2.1 contrast combinations (ratios computed by [`contrast_ratio`]):

| Background | Foreground | Ratio | Level |
|------------|-----------|-------|-------|
| Suco verde | Yandí | 10.7:1 | AAA |
| Suco verde | Nhandí | 8.3:1 | AAA |
| Suco verde | Genipina | 6.0:1 | AA |
| Brisa | Tinta de guerra | 7.3:1 | AAA |
| Tinta de guerra | Suco verde | 12.2:1 | AAA |
| Tinta de guerra | Brisa | 7.3:1 | AAA |
| Brisa | Yandí | 6.5:1 | AA |
| Jandí | Suco verde | 4.4:1 | AA Large |
| Tinta de guerra | Oby | 4.3:1 | AA Large |
| Oby | Suco verde | 2.8:1 | — |

> All ratios verified by `cargo test --features contrast`. Use [`contrast_ratio`], [`wcag_aa`], [`wcag_aaa`], and
> [`wcag_aa_large`] to validate your own combinations.

## Running Tests

```bash
# All tests with all features
cargo test --all-features

# Doc-tests only
cargo test --doc

# Contrast-specific tests
cargo test --features contrast

# Cross-format validation (values match TypeScript)
cargo test --test cross_format

# Coverage
cargo llvm-cov --all-features
```

## Package Structure

```
packages/rust/
├── Cargo.toml
├── src/
│   ├── lib.rs                 # Crate root (#![no_std])
│   ├── types.rs               # JandiColor, Rgb, Hsl, OxidationStage
│   ├── palette.rs             # 8 color constants + PALETTE + by_slug()
│   ├── contrast.rs            # WCAG 2.1 utilities (feature: contrast)
│   └── integrations/          # Feature-gated framework conversions
│       ├── mod.rs
│       ├── ratatui.rs
│       ├── iced.rs
│       ├── bevy.rs
│       ├── egui.rs
│       ├── crossterm.rs
│       ├── palette_crate.rs
│       └── serde_impl.rs
├── tests/
│   ├── palette_integrity.rs
│   ├── cross_format.rs
│   ├── contrast.rs
│   └── integrations.rs
└── examples/
    ├── print_palette.rs
    ├── ratatui_demo.rs
    └── iced_demo.rs
```

## Other Ecosystems

This palette is also available as:

- **npm:** [`@jandi/colors`](https://www.npmjs.com/package/@jandi/colors) — JavaScript/TypeScript
- **CSS/SCSS/JSON/Tailwind** — Design tokens for web projects
- **Swift/Kotlin** — Tokens for native mobile applications

See the [monorepo root](https://github.com/franciscotbjr/jandi-colors) for all available formats.

## License

MIT — use freely in personal and commercial projects.

---

*Uma paleta brasileira. Do fruto à tela.*
