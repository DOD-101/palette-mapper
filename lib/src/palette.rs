//! Items relating to color Palettes
//!
//! The main type is [`Palette`].
use image::Rgba;

#[cfg(feature = "serde")]
use {
    serde::{
        Serialize,
        de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
        ser::SerializeSeq,
    },
    std::fmt,
};

/// Helper macro for creating a [`Palette`]
///
/// ## Usage
///
///
/// ```
/// # use palette_mapper::color_pallete;
/// let p = color_pallete!(
///     [34, 63, 24],
///     [4, 3, 12, 50], // with alpha value
///     [20, 100, 203]
/// );
/// # use image::Rgba;
///
/// let p1 = vec![
///     Rgba::<u8>::from([34, 63, 24, 255]),
///     Rgba::<u8>::from([4, 3, 12, 50]),
///     Rgba::<u8>::from([20, 100, 203, 255]),
/// ].into();
///
/// assert_eq!(p, p1);
///
/// ```
#[macro_export]
macro_rules! color_pallete {
    ($([$red:expr, $green:expr, $blue:expr $(, $alpha:tt)?]),+) => {
        $crate::palette::Palette::from(
                                vec![
                                    $(
                                        $crate::rgba!(
                                                $red,
                                                $green,
                                                $blue
                                                $(, $alpha)?)),+])
    };
}

/// Helper macro for creating an [`Rgba`]
///
/// ## Usage
///
/// ```
/// # use image::Rgba;
/// # use palette_mapper::rgba;
/// assert_eq!(rgba!(100, 21, 98), Rgba::<u8>::from([100, 21, 98, 255]));
/// assert_eq!(rgba!(244, 104, 12, 200), Rgba::<u8>::from([244, 104, 12, 200]));
///
/// ```
#[macro_export]
macro_rules! rgba {
    ($red:expr, $green:expr, $blue:expr) => {
        $crate::rgba!($red, $green, $blue, 255)
    };
    ($red:expr, $green:expr, $blue:expr, $alpha:expr) => {
        image::Rgba::from([$red, $green, $blue, $alpha])
    };
}

/// A color palette
///
/// A color pallete is just a (curated) collection of colors usually designed to look nice
/// together.
///
/// ## Note on implementation
///
/// Currently this type is implemented internally with a [`Vec`] in the future this may change.
/// Because of this the internal type is not exposed. Hence traits such [`std::ops::Deref`] are
/// intentionally not implemented, since doing so might lead to a breaking change latter down the
/// road.
///
/// ### **See also**
///
/// - [`color_pallete`]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Palette(Vec<Rgba<u8>>);

impl Palette {
    /// Add a color to the pallete
    pub fn add_color(&mut self, col: Rgba<u8>) {
        self.0.push(col);
    }

    /// Returns an iterator over the slice.
    ///     
    /// The iterator yields all colors int the pallete from start to end.
    #[must_use]
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            this: self.0.iter(),
        }
    }

    /// Returns the amount of entries in the palette
    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the palette contains no entries
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a> IntoIterator for &'a Palette {
    type Item = &'a Rgba<u8>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for Palette {
    type Item = Rgba<u8>;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            this: self.0.into_iter(),
        }
    }
}

impl From<Vec<Rgba<u8>>> for Palette {
    fn from(value: Vec<Rgba<u8>>) -> Self {
        Self(value)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Palette {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;

        for e in self {
            seq.serialize_element(&e.0)?;
        }

        seq.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Palette {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PaletteVisitor;

        impl<'de> Visitor<'de> for PaletteVisitor {
            type Value = Palette;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of RGB ([u8; 3]) or RGBA ([u8; 4]) color arrays")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut colors = Vec::with_capacity(seq.size_hint().unwrap_or(0));

                while let Some(raw) = seq.next_element::<Vec<u8>>()? {
                    let rgba = match *raw.as_slice() {
                        [r, g, b] => Rgba::from([r, g, b, 255]),
                        [r, g, b, a] => Rgba::from([r, g, b, a]),
                        _ => {
                            return Err(de::Error::invalid_length(
                                raw.len(),
                                &"expected an array of length 3 (RGB) or 4 (RGBA)",
                            ));
                        }
                    };

                    colors.push(rgba);
                }

                Ok(Palette::from(colors))
            }
        }

        deserializer.deserialize_seq(PaletteVisitor)
    }
}

/// Immutable [Palette] iterator
///
/// ## Note on implementation
///
/// See [`Palette#note-on-implementation`]
pub struct Iter<'a> {
    /// Internal iterator, since this currently just wraps the type for a Vec
    this: std::slice::Iter<'a, Rgba<u8>>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Rgba<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        self.this.next()
    }
}

/// An iterator that moves out of a [Palette].
///
/// This struct is created by [`Palette::into_iter`].
///
/// ## Note on implementation
///
/// See [`Palette#note-on-implementation`]
pub struct IntoIter {
    /// Internal iterator, since this currently just wraps the type for a Vec
    this: std::vec::IntoIter<Rgba<u8>>,
}

impl Iterator for IntoIter {
    type Item = Rgba<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        self.this.next()
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[cfg(feature = "serde")]
    fn serde_roundtrip() {
        let p = color_pallete!([12, 45, 67, 200], [87, 212, 45]);

        let v = serde_json::to_value(&p).unwrap();

        assert_eq!(p, serde_json::from_value(v).unwrap());
    }
}
