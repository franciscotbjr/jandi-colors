use core::fmt;

/// Oxidation stages of genipin, from fresh pulp extract to maximum ritual concentration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OxidationStage {
    /// Pre-oxidation — fresh pulp extract, no hydrolysis yet
    Pre,
    /// First contact with atmospheric oxygen
    Initial,
    /// Active chromophore formation, incomplete protein cross-linking
    Partial,
    /// Complete oxidation at standard concentration
    Full,
    /// Concentrated extract with excess free genipin
    Saturated,
    /// Multiple immersion layers, cumulative density
    Deep,
    /// Extract reduced by evaporation for long-duration paint
    Concentrated,
    /// Maximum ritual concentration, indistinguishable from black
    Max,
}

impl fmt::Display for OxidationStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pre => write!(f, "pre"),
            Self::Initial => write!(f, "initial"),
            Self::Partial => write!(f, "partial"),
            Self::Full => write!(f, "full"),
            Self::Saturated => write!(f, "saturated"),
            Self::Deep => write!(f, "deep"),
            Self::Concentrated => write!(f, "concentrated"),
            Self::Max => write!(f, "max"),
        }
    }
}

/// RGB color values with 8-bit channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb {
    /// Red channel (0–255)
    pub r: u8,
    /// Green channel (0–255)
    pub g: u8,
    /// Blue channel (0–255)
    pub b: u8,
}

/// HSL color values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hsl {
    /// Hue in degrees (0–360)
    pub h: u16,
    /// Saturation percentage (0–100)
    pub s: u8,
    /// Lightness percentage (0–100)
    pub l: u8,
}

/// A color from the Jandí palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JandiColor {
    /// Human-readable name (e.g. "Suco verde")
    pub name: &'static str,
    /// Unique slug identifier (e.g. "jandi-suco-verde")
    pub slug: &'static str,
    /// Hex color value including `#` (e.g. "#C8D5CC")
    pub hex: &'static str,
    /// RGB color channels
    pub rgb: Rgb,
    /// HSL color values
    pub hsl: Hsl,
    /// Oxidation stage in the genipin reaction
    pub oxidation_stage: OxidationStage,
    /// Ethnobotanical description
    pub description: &'static str,
}

impl fmt::Display for JandiColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.hex)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use std::string::ToString;

    #[test]
    fn test_copy_clone() {
        let c = JandiColor {
            name: "Test",
            slug: "test",
            hex: "#000000",
            rgb: Rgb { r: 0, g: 0, b: 0 },
            hsl: Hsl { h: 0, s: 0, l: 0 },
            oxidation_stage: OxidationStage::Pre,
            description: "",
        };
        let c2 = c;
        assert_eq!(c.name, c2.name);
    }

    #[test]
    fn test_eq_hash() {
        let a = JandiColor {
            name: "A",
            slug: "a",
            hex: "#000000",
            rgb: Rgb { r: 0, g: 0, b: 0 },
            hsl: Hsl { h: 0, s: 0, l: 0 },
            oxidation_stage: OxidationStage::Pre,
            description: "",
        };
        let b = JandiColor {
            name: "A",
            slug: "a",
            hex: "#000000",
            rgb: Rgb { r: 0, g: 0, b: 0 },
            hsl: Hsl { h: 0, s: 0, l: 0 },
            oxidation_stage: OxidationStage::Pre,
            description: "",
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_display_jandi_color() {
        let c = JandiColor {
            name: "Test Color",
            slug: "test",
            hex: "#ABCDEF",
            rgb: Rgb {
                r: 0xAB,
                g: 0xCD,
                b: 0xEF,
            },
            hsl: Hsl { h: 0, s: 0, l: 0 },
            oxidation_stage: OxidationStage::Full,
            description: "",
        };
        assert_eq!(c.to_string(), "Test Color (#ABCDEF)");
    }

    #[test]
    fn test_display_oxidation_stage() {
        assert_eq!(OxidationStage::Pre.to_string(), "pre");
        assert_eq!(OxidationStage::Initial.to_string(), "initial");
        assert_eq!(OxidationStage::Partial.to_string(), "partial");
        assert_eq!(OxidationStage::Full.to_string(), "full");
        assert_eq!(OxidationStage::Saturated.to_string(), "saturated");
        assert_eq!(OxidationStage::Deep.to_string(), "deep");
        assert_eq!(OxidationStage::Concentrated.to_string(), "concentrated");
        assert_eq!(OxidationStage::Max.to_string(), "max");
    }

    #[test]
    fn test_debug_format() {
        let c = JandiColor {
            name: "Dbg",
            slug: "dbg",
            hex: "#000000",
            rgb: Rgb { r: 0, g: 0, b: 0 },
            hsl: Hsl { h: 0, s: 0, l: 0 },
            oxidation_stage: OxidationStage::Pre,
            description: "",
        };
        let debug = std::format!("{:?}", c);
        assert!(debug.contains("Dbg"));
    }
}
