//! Conversions between different color representations and color spaces
//!
//! These implementations are, while tested to be correct, not scientifically accurate.
//!
//! This module assumes as stated in the docs of [`Rgb`] (and [`Rgba`]) that these values are sRGB.
use image::{Rgb, Rgba};
use thiserror::Error;

/// The XYZ tristimulus values for the D65 reference white of the 2° standard observer of the CIE1931
/// standard.
///
/// Values taken from: <https://en.wikipedia.org/wiki/Standard_illuminant#D65_values>
const D54_STANDARD_2_OBSERVER_TRISTIMULUS: Xyz = Xyz([0.95047, 1.0, 1.08883]);

/// Matrix used for conversion from sRGB to XYZ
///
/// This is the D65 sRGB to XYZ matrix taken from <http://brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html>
const D65_S_RGB_MATRIX: [[f32; 3]; 3] = [
    [0.412_456_4, 0.357_576_1, 0.180_437_5], // row
    [0.212_672_9, 0.715_152_2, 0.072_175_0], // row
    [0.019_333_9, 0.119_192, 0.950_304_1],   // row
];

/// Junction point constant given the CIE standards
const EPSILON: f32 = 216.0 / 24389.0;
/// Constant given by the CIE standards
const K: f32 = 24389.0 / 27.0;

/// Errors generated when attempting conversions
#[derive(Debug, Error)]
pub enum ConversionError {
    /// The inputted value was too long to allow for conversion
    #[error("The input value was too long.")]
    InputTooLong,
    /// The inputted value was too short to allow for conversion
    #[error("The input value was too short to allow for conversion")]
    InputTooShort,
}

/// Trait to allow easier conversion to [`Rgb`] and [`Rgba`]
pub trait RgbConversionExt {
    /// Convert `self` to [`Rgb<u8>`]
    fn to_rgb(self) -> Rgb<u8>;
    /// Convert `self` to [`Rgba<u8>`]
    #[allow(
        dead_code,
        reason = "Might be useful in the future, even if it's not used right now."
    )]
    fn to_rgba(self) -> Rgba<u8>;
}

impl RgbConversionExt for Rgb<u8> {
    fn to_rgb(self) -> Rgb<u8> {
        self
    }

    fn to_rgba(self) -> Rgba<u8> {
        Rgba::from([self[0], self[1], self[2], 255])
    }
}

impl RgbConversionExt for Rgba<u8> {
    fn to_rgb(self) -> Rgb<u8> {
        Rgb::from([self[0], self[1], self[2]])
    }

    fn to_rgba(self) -> Rgba<u8> {
        self
    }
}

/// Represents a point in XYZ color space
#[derive(Debug, PartialEq)]
pub struct Xyz(pub(crate) [f32; 3]);

impl From<[f32; 3]> for Xyz {
    fn from(value: [f32; 3]) -> Self {
        Self(value)
    }
}

impl TryFrom<Vec<f32>> for Xyz {
    type Error = ConversionError;
    fn try_from(value: Vec<f32>) -> Result<Self, Self::Error> {
        if value.len() > 3 {
            return Err(ConversionError::InputTooLong);
        }

        if value.len() < 3 {
            return Err(ConversionError::InputTooShort);
        }

        Ok(Self::from([value[0], value[1], value[2]]))
    }
}

impl From<Rgb<u8>> for Xyz {
    /// Convert from sRGB to Xyz color space
    fn from(value: Rgb<u8>) -> Self {
        let gama_corrected = value.0.map(|v| {
            let normalized = f32::from(v) / 255.0;

            if normalized <= 0.04045 {
                normalized / 12.92
            } else {
                ((normalized + 0.055) / 1.055).powf(2.4)
            }
        });

        D65_S_RGB_MATRIX
            .iter()
            .map(|row| {
                row.iter()
                    .zip(gama_corrected.iter())
                    .fold(0_f32, |mut total, entry| {
                        total += entry.0 * entry.1;

                        total
                    })
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("We know the vec has len of 3.")
    }
}

impl From<Rgba<u8>> for Xyz {
    fn from(value: Rgba<u8>) -> Self {
        value.to_rgb().into()
    }
}

/// A color represented in CIELAB color space
///
/// See:
///
/// - <https://en.wikipedia.org/wiki/CIELAB_color_space>
#[derive(Debug, PartialEq)]
pub struct Lab(pub(crate) [f32; 3]);

impl From<Xyz> for Lab {
    fn from(value: Xyz) -> Self {
        const WHITE: Xyz = D54_STANDARD_2_OBSERVER_TRISTIMULUS;

        let f = |x: f32| {
            if x > EPSILON {
                x.cbrt()
            } else {
                K.mul_add(x, 16.0) / 116.0
            }
        };

        let fx = f(value.0[0] / WHITE.0[0]);
        let fy = f(value.0[1] / WHITE.0[1]);
        let fz = f(value.0[2] / WHITE.0[2]);

        let l = 116.0f32.mul_add(fy, -16.0);
        let a = 500.0 * (fx - fy);
        let b = 200.0 * (fy - fz);

        Self([l, a, b])
    }
}

impl From<[f32; 3]> for Lab {
    fn from(value: [f32; 3]) -> Self {
        Self(value)
    }
}

impl From<Rgb<u8>> for Lab {
    fn from(value: Rgb<u8>) -> Self {
        Self::from(Xyz::from(value))
    }
}

#[cfg(test)]
mod test {
    use image::Rgb;

    use crate::conversions::{Lab, Xyz};

    macro_rules! assert_eq_within {
        ($left:expr, $right:expr) => {
            assert_eq_within!($left, $right, 4_u8)
        };

        ($left:expr, $right:expr, $within:expr) => {
            #[allow(
                unused_must_use,
                reason = "We just use this to ensure left and right are of eq-able types"
            )]
            #[allow(clippy::float_cmp, reason = "We round this on purpose")]
            {
                $left == $right;

                let mut left = $left.0;
                let mut right = $right.0;

                let within = 10_f32.powf(f32::from($within));

                for v in left.iter_mut() {
                    *v = (*v * within).round() / within;
                }

                for v in right.iter_mut() {
                    *v = (*v * within).round() / within;
                }

                assert_eq!(left, right);
            }
        };
    }

    // all rgb to xyz reference values are taken from https://formatfuse.com/convert/color/rgb-to-xyz/ and compared in similar online tools

    #[test]
    fn rgb_to_xyz_white() {
        assert_eq_within!(
            Xyz::from(Rgb::<u8>::from([255, 255, 255])),
            Xyz::from([0.9505, 1.0, 1.0888])
        );
    }

    #[test]
    fn rgb_to_xyz_black() {
        assert_eq_within!(
            Xyz::from(Rgb::<u8>::from([0, 0, 0])),
            Xyz::from([0.0, 0.0, 0.0])
        );
    }

    #[test]
    fn rgb_to_xyz_palevioletred() {
        // arbitrarily chosen value
        //
        // Name from: https://colordesigner.io/color-name-finder
        assert_eq_within!(
            Xyz::from(Rgb::<u8>::from([123, 45, 78])),
            Xyz::from([0.1048, 0.0664, 0.0794])
        );
    }

    #[test]
    fn rgb_to_lab_white() {
        assert_eq!(
            Lab::from(Xyz::from(Rgb::<u8>::from([255, 255, 255]))),
            Lab::from([100.0, 0.0, 0.0]),
        );
    }

    #[test]
    fn rgb_to_lab_black() {
        assert_eq_within!(
            Xyz::from(Rgb::<u8>::from([0, 0, 0])),
            Xyz::from([0.0, 0.0, 0.0])
        );
    }

    #[test]
    fn rgb_to_lab_palevioletred() {
        assert_eq_within!(
            Lab::from(Xyz::from(Rgb::<u8>::from([123, 45, 78]))),
            Lab::from([30.9703, 37.3212, -2.5585])
        );
    }
}
