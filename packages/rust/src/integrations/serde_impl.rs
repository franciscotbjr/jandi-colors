use crate::by_slug;
use crate::types::{JandiColor, OxidationStage};
use core::fmt;
use serde::{Deserialize, Serialize, de::MapAccess};

#[derive(Serialize)]
struct SerdeRgb {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Serialize)]
struct SerdeHsl {
    h: u16,
    s: u8,
    l: u8,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SerdeJandiColor<'a> {
    name: &'a str,
    slug: &'a str,
    hex: &'a str,
    rgb: SerdeRgb,
    hsl: SerdeHsl,
    oxidation_stage: OxidationStage,
    description: &'a str,
}

impl Serialize for JandiColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let helper = SerdeJandiColor {
            name: self.name,
            slug: self.slug,
            hex: self.hex,
            rgb: SerdeRgb {
                r: self.rgb.r,
                g: self.rgb.g,
                b: self.rgb.b,
            },
            hsl: SerdeHsl {
                h: self.hsl.h,
                s: self.hsl.s,
                l: self.hsl.l,
            },
            oxidation_stage: self.oxidation_stage,
            description: self.description,
        };
        helper.serialize(serializer)
    }
}

struct JandiColorVisitor;

impl<'de> serde::de::Visitor<'de> for JandiColorVisitor {
    type Value = JandiColor;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a JandiColor object with a valid slug")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut slug: Option<&'de str> = None;
        while let Some(key) = map.next_key::<&'de str>()? {
            match key {
                "slug" => {
                    slug = Some(map.next_value()?);
                }
                _ => {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
        }
        let slug = slug.ok_or_else(|| serde::de::Error::missing_field("slug"))?;
        by_slug(slug)
            .copied()
            .ok_or_else(|| serde::de::Error::custom(format_args!("unknown slug: {slug}")))
    }
}

/// Deserialization identifies the canonical palette color by its `slug` field.
///
/// All other fields in the input JSON (`name`, `hex`, `rgb`, `hsl`,
/// `oxidationStage`, `description`) are ignored — only the slug is used
/// to resolve the matching [`JandiColor`] constant.  This means a JSON
/// payload with a valid slug but incorrect colour data will deserialise
/// successfully and return the **canonical** palette value.
impl<'de> Deserialize<'de> for JandiColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("JandiColor", &["slug"], JandiColorVisitor)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum SerdeOxidationStage {
    Pre,
    Initial,
    Partial,
    Full,
    Saturated,
    Deep,
    Concentrated,
    Max,
}

impl Serialize for OxidationStage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let helper: SerdeOxidationStage = match self {
            OxidationStage::Pre => SerdeOxidationStage::Pre,
            OxidationStage::Initial => SerdeOxidationStage::Initial,
            OxidationStage::Partial => SerdeOxidationStage::Partial,
            OxidationStage::Full => SerdeOxidationStage::Full,
            OxidationStage::Saturated => SerdeOxidationStage::Saturated,
            OxidationStage::Deep => SerdeOxidationStage::Deep,
            OxidationStage::Concentrated => SerdeOxidationStage::Concentrated,
            OxidationStage::Max => SerdeOxidationStage::Max,
        };
        helper.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for OxidationStage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let helper = SerdeOxidationStage::deserialize(deserializer)?;
        Ok(match helper {
            SerdeOxidationStage::Pre => OxidationStage::Pre,
            SerdeOxidationStage::Initial => OxidationStage::Initial,
            SerdeOxidationStage::Partial => OxidationStage::Partial,
            SerdeOxidationStage::Full => OxidationStage::Full,
            SerdeOxidationStage::Saturated => OxidationStage::Saturated,
            SerdeOxidationStage::Deep => OxidationStage::Deep,
            SerdeOxidationStage::Concentrated => OxidationStage::Concentrated,
            SerdeOxidationStage::Max => OxidationStage::Max,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::PRIMARY;
    use crate::types::{JandiColor, OxidationStage};

    #[test]
    fn test_jandi_color_roundtrip() {
        let json = serde_json::to_string(&PRIMARY).expect("serialize");
        let deserialized: JandiColor = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(PRIMARY.name, deserialized.name);
        assert_eq!(PRIMARY.slug, deserialized.slug);
        assert_eq!(PRIMARY.hex, deserialized.hex);
        assert_eq!(PRIMARY.rgb, deserialized.rgb);
        assert_eq!(PRIMARY.hsl, deserialized.hsl);
        assert_eq!(PRIMARY.oxidation_stage, deserialized.oxidation_stage);
    }

    #[test]
    fn test_json_shape_matches_typescript() {
        let json = serde_json::to_string(&PRIMARY).expect("serialize");
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(
            v["rgb"].is_object(),
            "rgb should be an object, not an array"
        );
        assert_eq!(v["rgb"]["r"], 61);
        assert_eq!(v["rgb"]["g"], 95);
        assert_eq!(v["rgb"]["b"], 128);
        assert!(
            v["hsl"].is_object(),
            "hsl should be an object, not an array"
        );
        assert_eq!(v["hsl"]["h"], 210);
        assert_eq!(v["hsl"]["s"], 35);
        assert_eq!(v["hsl"]["l"], 37);
        assert_eq!(v["oxidationStage"], "full");
    }

    #[test]
    fn test_oxidation_stage_camel_case() {
        let json = serde_json::to_string(&OxidationStage::Full).expect("serialize");
        assert_eq!(json, r#""full""#);
        let deserialized: OxidationStage = serde_json::from_str(r#""full""#).expect("deserialize");
        assert_eq!(deserialized, OxidationStage::Full);
    }
}
