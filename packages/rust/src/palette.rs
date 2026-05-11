use crate::types::{Hsl, JandiColor, OxidationStage, Rgb};

/// `#C8D5CC` — Pre-oxidation, fresh pulp extract
pub const SUCO_VERDE: JandiColor = JandiColor {
    name: "Suco verde",
    slug: "jandi-suco-verde",
    hex: "#C8D5CC",
    rgb: Rgb {
        r: 200,
        g: 213,
        b: 204,
    },
    hsl: Hsl {
        h: 138,
        s: 13,
        l: 81,
    },
    oxidation_stage: OxidationStage::Pre,
    description: "Polpa recém-extraída do fruto verde. O geniposídeo foi liberado pela trituração, mas a β-glucosidase ainda não completou a hidrólise. Nenhuma reação com proteínas ocorreu.",
};

/// `#8FA7B3` — Light oxidation, first atmospheric contact
pub const BRISA: JandiColor = JandiColor {
    name: "Brisa",
    slug: "jandi-brisa",
    hex: "#8FA7B3",
    rgb: Rgb {
        r: 143,
        g: 167,
        b: 179,
    },
    hsl: Hsl {
        h: 200,
        s: 19,
        l: 63,
    },
    oxidation_stage: OxidationStage::Initial,
    description: "Primeiro contato com o oxigênio atmosférico. A genipina livre começa a reagir com aminoácidos, formando os primeiros intermediários amarelo-acinzentados. A cor é sutil — como a brisa que antecede a mudança.",
};

/// `#5D7F96` — Tupi blue (oby = blue/green in Tupi)
pub const OBY: JandiColor = JandiColor {
    name: "Oby",
    slug: "jandi-oby",
    hex: "#5D7F96",
    rgb: Rgb {
        r: 93,
        g: 127,
        b: 150,
    },
    hsl: Hsl {
        h: 204,
        s: 23,
        l: 48,
    },
    oxidation_stage: OxidationStage::Partial,
    description: "O azul emerge. Os cromóforos de genipina-aminoácido estão se formando ativamente — a reticulação proteica está em progresso mas incompleta. O tom é reconhecivelmente azul mas ainda translúcido.",
};

/// `#3D5F80` — Primary color, the revealed dye
pub const PRIMARY: JandiColor = JandiColor {
    name: "Jandí",
    slug: "jandi-primary",
    hex: "#3D5F80",
    rgb: Rgb {
        r: 61,
        g: 95,
        b: 128,
    },
    hsl: Hsl {
        h: 210,
        s: 35,
        l: 37,
    },
    oxidation_stage: OxidationStage::Full,
    description: "Oxidação completa em concentração padrão. Todos os sítios de ligação genipina-proteína foram ocupados. Este é o azul que os povos indígenas obtêm na primeira aplicação sobre a pele — a tinta revelada.",
};

/// `#2C4A6E` — Deep indigo, full genipin oxidation
pub const GENIPINA: JandiColor = JandiColor {
    name: "Genipina",
    slug: "jandi-genipina",
    hex: "#2C4A6E",
    rgb: Rgb {
        r: 44,
        g: 74,
        b: 110,
    },
    hsl: Hsl {
        h: 213,
        s: 43,
        l: 30,
    },
    oxidation_stage: OxidationStage::Saturated,
    description: "Extrato concentrado com excesso de genipina livre. A proporção genipina/proteína é maior que a estequiométrica — camadas sobrepostas de pigmento produzem índigo natural profundo.",
};

/// `#1E3452` — Night blue, multiple immersions
pub const NHANDI: JandiColor = JandiColor {
    name: "Nhandí",
    slug: "jandi-nhandi",
    hex: "#1E3452",
    rgb: Rgb {
        r: 30,
        g: 52,
        b: 82,
    },
    hsl: Hsl {
        h: 215,
        s: 46,
        l: 22,
    },
    oxidation_stage: OxidationStage::Deep,
    description: "Múltiplas imersões. A pele é pintada, seca e pintada novamente — cada camada adiciona densidade ao pigmento reticulado na epiderme. O efeito cumulativo produz um azul-noite que absorve quase toda a luz visível.",
};

/// `#11203A` — Near-black, high concentration
pub const YANDI: JandiColor = JandiColor {
    name: "Yandí",
    slug: "jandi-yandi",
    hex: "#11203A",
    rgb: Rgb {
        r: 17,
        g: 32,
        b: 58,
    },
    hsl: Hsl {
        h: 218,
        s: 55,
        l: 15,
    },
    oxidation_stage: OxidationStage::Concentrated,
    description: "Extrato reduzido por evaporação. O líquido foi fervido ou exposto ao sol para concentrar a genipina — técnica usada para preparar tinta de longa duração para cerâmica e tecidos.",
};

/// `#0A1424` — War paint, maximum ritual concentration
pub const TINTA_GUERRA: JandiColor = JandiColor {
    name: "Tinta de guerra",
    slug: "jandi-tinta-guerra",
    hex: "#0A1424",
    rgb: Rgb {
        r: 10,
        g: 20,
        b: 36,
    },
    hsl: Hsl {
        h: 217,
        s: 57,
        l: 9,
    },
    oxidation_stage: OxidationStage::Max,
    description: "Concentração máxima ritual. Extrato reduzido ao limite, aplicado em múltiplas camadas sobre pele escarificada para maximizar a penetração proteica. Indistinguível do preto a olho nu — só revela seu azul sob luz rasante.",
};

/// Alias for `PRIMARY`, matching the TypeScript `jandi = primary` export
pub const JANDI: JandiColor = PRIMARY;

/// Current crate version
pub const VERSION: &str = "0.1.0";

/// All 8 colors ordered lightest to darkest by lightness value
pub const PALETTE: [JandiColor; 8] = [
    SUCO_VERDE,
    BRISA,
    OBY,
    PRIMARY,
    GENIPINA,
    NHANDI,
    YANDI,
    TINTA_GUERRA,
];

const fn str_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// Look up a color by its slug.
///
/// Returns `None` if no color matches the given slug.
pub const fn by_slug(slug: &str) -> Option<&'static JandiColor> {
    if str_eq(slug, "jandi-suco-verde") {
        Some(&SUCO_VERDE)
    } else if str_eq(slug, "jandi-brisa") {
        Some(&BRISA)
    } else if str_eq(slug, "jandi-oby") {
        Some(&OBY)
    } else if str_eq(slug, "jandi-primary") {
        Some(&PRIMARY)
    } else if str_eq(slug, "jandi-genipina") {
        Some(&GENIPINA)
    } else if str_eq(slug, "jandi-nhandi") {
        Some(&NHANDI)
    } else if str_eq(slug, "jandi-yandi") {
        Some(&YANDI)
    } else if str_eq(slug, "jandi-tinta-guerra") {
        Some(&TINTA_GUERRA)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_by_slug_found() {
        let color = by_slug("jandi-primary").expect("slug should be found");
        assert_eq!(color.hex, "#3D5F80");
    }

    #[test]
    fn test_by_slug_not_found() {
        assert!(by_slug("nonexistent").is_none());
    }

    #[test]
    fn test_by_slug_empty_string() {
        assert!(by_slug("").is_none());
    }

    #[test]
    fn test_jandi_alias() {
        assert_eq!(JANDI.hex, PRIMARY.hex);
        assert_eq!(JANDI.slug, PRIMARY.slug);
        assert_eq!(JANDI.name, PRIMARY.name);
    }

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.1.0");
    }

    #[test]
    fn test_palette_len() {
        assert_eq!(PALETTE.len(), 8);
    }

    #[test]
    fn test_names_unique() {
        let mut names: [&str; 8] = [""; 8];
        for (i, color) in PALETTE.iter().enumerate() {
            names[i] = color.name;
        }
        let mut i = 0;
        while i < names.len() {
            let mut j = i + 1;
            while j < names.len() {
                assert!(!str_eq(names[i], names[j]), "duplicate name: {}", names[i]);
                j += 1;
            }
            i += 1;
        }
    }

    #[test]
    fn test_str_eq() {
        assert!(str_eq("hello", "hello"));
        assert!(!str_eq("hello", "world"));
        assert!(!str_eq("hello", "hell"));
        assert!(!str_eq("hell", "hello"));
    }
}
