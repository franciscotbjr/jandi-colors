use crate::types::JandiColor;

impl From<JandiColor> for crossterm::style::Color {
    fn from(color: JandiColor) -> Self {
        crossterm::style::Color::Rgb {
            r: color.rgb.r,
            g: color.rgb.g,
            b: color.rgb.b,
        }
    }
}

impl From<&JandiColor> for crossterm::style::Color {
    fn from(color: &JandiColor) -> Self {
        crossterm::style::Color::Rgb {
            r: color.rgb.r,
            g: color.rgb.g,
            b: color.rgb.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::PRIMARY;
    #[test]
    fn test_crossterm_conversion_owned() {
        let color: crossterm::style::Color = PRIMARY.into();
        assert_eq!(
            color,
            crossterm::style::Color::Rgb {
                r: 61,
                g: 95,
                b: 128
            }
        );
    }

    #[test]
    fn test_crossterm_conversion_borrowed() {
        let color: crossterm::style::Color = (&PRIMARY).into();
        assert_eq!(
            color,
            crossterm::style::Color::Rgb {
                r: 61,
                g: 95,
                b: 128
            }
        );
    }
}
