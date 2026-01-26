//! Palettes auto-generated from the base16/24 themes
//!
//! Source: <https://github.com/tinted-theming/schemes>

#![allow(
    clippy::allow_attributes_without_reason,
    reason = "This is generated code"
)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::too_many_lines)]

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[cfg(test)]
mod test {
    use super::{Base16, Base24};
    use palette_mapper::Palette;
    use strum::IntoEnumIterator;

    #[test]
    /// Ensures base16 themes are actually 16 colors and base24 are actually 24 colors
    fn ensure_proper_bases() {
        for theme in Base16::iter() {
            assert_eq!(Palette::from(theme).len(), 16);
        }

        for theme in Base24::iter() {
            assert_eq!(Palette::from(theme).len(), 24);
        }
    }
}
