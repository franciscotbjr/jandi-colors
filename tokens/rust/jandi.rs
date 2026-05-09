//! # Jandí Colors
//!
//! Color palette derived from the blue pigment of jenipapo (*Genipa americana*),
//! the ancestral dye of Brazil's indigenous peoples.
//!
//! Each color is represented as `(r, g, b)` tuple and as a hex `&str`.
//!
//! ```rust
//! use jandi_colors::{JANDI_PRIMARY, JANDI_PRIMARY_HEX};
//!
//! let (r, g, b) = JANDI_PRIMARY;
//! assert_eq!(JANDI_PRIMARY_HEX, "#3D5F80");
//! ```

/// #C8D5CC — Pre-oxidation, fresh pulp extract
pub const SUCO_VERDE: (u8, u8, u8) = (200, 213, 204);
pub const SUCO_VERDE_HEX: &str = "#C8D5CC";

/// #8FA7B3 — Light oxidation, first atmospheric contact
pub const BRISA: (u8, u8, u8) = (143, 167, 179);
pub const BRISA_HEX: &str = "#8FA7B3";

/// #5D7F96 — Tupi blue (oby = blue/green in Tupi)
pub const OBY: (u8, u8, u8) = (93, 127, 150);
pub const OBY_HEX: &str = "#5D7F96";

/// #3D5F80 — Primary color, the revealed dye
pub const JANDI_PRIMARY: (u8, u8, u8) = (61, 95, 128);
pub const JANDI_PRIMARY_HEX: &str = "#3D5F80";

/// #2C4A6E — Deep indigo, full genipin oxidation
pub const GENIPINA: (u8, u8, u8) = (44, 74, 110);
pub const GENIPINA_HEX: &str = "#2C4A6E";

/// #1E3452 — Night blue, multiple immersions
pub const NHANDI: (u8, u8, u8) = (30, 52, 82);
pub const NHANDI_HEX: &str = "#1E3452";

/// #11203A — Near-black, high concentration
pub const YANDI: (u8, u8, u8) = (17, 32, 58);
pub const YANDI_HEX: &str = "#11203A";

/// #0A1424 — War paint, maximum ritual concentration
pub const TINTA_GUERRA: (u8, u8, u8) = (10, 20, 36);
pub const TINTA_GUERRA_HEX: &str = "#0A1424";

/// All colors ordered lightest to darkest
pub const ALL: [(u8, u8, u8); 8] = [
    SUCO_VERDE, BRISA, OBY, JANDI_PRIMARY,
    GENIPINA, NHANDI, YANDI, TINTA_GUERRA,
];

/// All hex values ordered lightest to darkest
pub const ALL_HEX: [&str; 8] = [
    SUCO_VERDE_HEX, BRISA_HEX, OBY_HEX, JANDI_PRIMARY_HEX,
    GENIPINA_HEX, NHANDI_HEX, YANDI_HEX, TINTA_GUERRA_HEX,
];
