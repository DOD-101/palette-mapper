//! Module for serde-related items for [`Palette`]
use super::Palette;
use image::Rgba;

use serde::{
    Deserialize, Serialize,
    de::{self, Deserializer},
    ser::{SerializeSeq, Serializer},
};

#[derive(Deserialize)]
#[serde(untagged)]
/// Accepted representations a color in a palette file
enum ColorRepr {
    /// As a hex string, eg.: `#ffeecc`
    Hex(String),
    /// As an rbg(a) array, eg.: `[144, 234, 12]`
    Array(Vec<u8>),
}

/// Parses a hex color into an rgba array
///
/// Accepted forms:
///
/// ```no_test
/// parse_hex_color("#ff00ff")
/// parse_hex_color("#ff00ff00")
/// ```
fn parse_hex_color(s: &str) -> Option<[u8; 4]> {
    let s = s.strip_prefix('#')?;
    match s.len() {
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            Some([r, g, b, 255])
        }
        8 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            let alpha = u8::from_str_radix(&s[6..8], 16).ok()?;
            Some([r, g, b, alpha])
        }
        _ => None,
    }
}

/// Helper function to convert an rgba array into a hex string
fn to_hex(rgba: [u8; 4]) -> String {
    if rgba[3] == 255 {
        format!("#{:02X}{:02X}{:02X}", rgba[0], rgba[1], rgba[2])
    } else {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            rgba[0], rgba[1], rgba[2], rgba[3]
        )
    }
}

impl Serialize for Palette {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;

        for color in self {
            seq.serialize_element(&to_hex(color.0))?;
        }

        seq.end()
    }
}

impl<'de> Deserialize<'de> for Palette {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: Vec<ColorRepr> = Vec::deserialize(deserializer)?;

        let mut colors = Vec::with_capacity(raw.len());

        for entry in raw {
            let rgba = Rgba::from(match entry {
                ColorRepr::Hex(s) => parse_hex_color(&s).ok_or_else(|| {
                    de::Error::custom("invalid hex color (expected #RRGGBB or #RRGGBBAA)")
                })?,
                ColorRepr::Array(v) => match *v.as_slice() {
                    [r, g, b] => [r, g, b, 255],
                    [r, g, b, alpha] => [r, g, b, alpha],
                    _ => {
                        return Err(de::Error::custom(
                            "color array must be [r,g,b] or [r,g,b,a]",
                        ));
                    }
                },
            });

            colors.push(rgba);
        }

        Ok(Self::from(colors))
    }
}

#[cfg(test)]
mod test {
    use super::parse_hex_color;
    use crate::color_palette;

    #[test]
    fn parse_hex_colors() {
        assert_eq!(parse_hex_color("#ff00ff"), Some([255, 0, 255, 255]));
        assert_eq!(parse_hex_color("#ff00ff00"), Some([255, 0, 255, 0]));
    }

    #[test]
    fn palette_serde_roundtrip() {
        let p = color_palette!([12, 45, 67, 200], [87, 212, 45]);

        let v = serde_json::to_value(&p).unwrap();

        assert_eq!(p, serde_json::from_value(v).unwrap());
    }

    #[test]
    fn palette_serialize() {
        let p = color_palette!([12, 45, 67, 200], [87, 212, 45]);

        let v = serde_json::to_string(&p);

        assert_eq!(v.unwrap(), r##"["#0C2D43C8","#57D42D"]"##);
    }
}
