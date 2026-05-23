//! Items relating to determining the "distance" between two colors
//!
//! ## What is a "distance" in this context?
//!
//! The distance between colors can be understood as how "similar" colors are.
//! If the distance is very small the color are very similar. If it is large they are very
//! different.
//!
//! Example:
//!      `#ffffff` and `#eeeeee` are more similar than `#ffffff` and `#000000`, so their distance
//!     is less.
//!
//! ## Determining the distance between colors
//!
//! There are many different approaches to determining the "distance" between colors.
//! For this purpose the [`DistanceAlgorithm`] trait exists. Any type implementing this trait ca be
//! used to calculate the distance between two colors.
//!
//! See the implementors of [`DistanceAlgorithm`] for different ways of calculating the distance.
//!
//! ### See also
//!
//! - [`Distance`]
//!
//! - [`Algorithms`]
use std::marker::PhantomData;

use image::Rgba;

use crate::conversions::{Lab, RgbConversionExt};

/// Trait representing an algorithm used to calculate the distance between two colors
///
/// ## Implementing this trait
///
/// When implementing this trait it is not relevant what the concrete values returned by
/// [`DistanceAlgorithm::distance`] are. They are never exposed to the user directly. They must
/// merely be a consistent measurement of how close two colors are to one another.
///
/// This means one Algorithm may return values in the range `0-100` while another uses the entire
/// range of [`u32`] values. As long as the values returned allow for comparing how close (or
/// similar) two colors are both implementations ok.
pub trait DistanceAlgorithm {
    /// Function used to determine the distance of two colors
    fn distance(&self, left: &Rgba<u8>, right: &Rgba<u8>) -> u32;
}

/// A distance between two colors
///
/// See [module level docs](`self`)
#[derive(Default, Debug)]
pub struct Distance<A: DistanceAlgorithm = EuclideanDistance> {
    /// The measured colors
    distance: u32,
    /// Marker for the algorithm used
    algorithm: PhantomData<A>,
}

impl<A: DistanceAlgorithm> PartialEq for Distance<A> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<A: DistanceAlgorithm> Eq for Distance<A> {}

impl<A: DistanceAlgorithm> PartialOrd for Distance<A> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<A: DistanceAlgorithm> Ord for Distance<A> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl<A: DistanceAlgorithm> Distance<A> {
    /// Create a new [`Distance`]
    ///
    /// The two points passed are calculated with the given algorithm
    #[must_use]
    pub fn new(left: &Rgba<u8>, right: &Rgba<u8>, algorithm: &A) -> Self {
        Self {
            distance: algorithm.distance(left, right),
            algorithm: PhantomData,
        }
    }

    /// Create a new [`Distance`] with the maximum value
    #[must_use]
    pub const fn new_max() -> Self {
        Self {
            distance: u32::MAX,
            algorithm: PhantomData,
        }
    }

    /// Create a new [`Distance`] with the minimum value
    #[must_use]
    pub const fn new_min() -> Self {
        Self {
            distance: u32::MIN,
            algorithm: PhantomData,
        }
    }
}

palette_mapper_macros::algorithms! {
    /// [Euclidean distance](https://en.wikipedia.org/wiki/Euclidean_distance)
    EuclideanDistance

    /// [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry)
    ManhattanDistance

    /// [CIE76](https://en.wikipedia.org/wiki/Color_difference#CIE76)
    ///
    /// This was the first formula used by the CIE to calculate Delta E using.
    /// It is in essence the [`EuclideanDistance`] in CIELAB coordinates.
    #[NoAlpha]
    CIE76

    /// A combination of [`EuclideanDistance`] and [`ManhattanDistance`] in die CIELAB color space
    #[NoAlpha]
    CIEHybrid

    /// [CIEDE2000](https://en.wikipedia.org/wiki/Color_difference#CIEDE2000)
    ///
    /// This is the most perceptually accurate formula, but also the most complicated and therefore
    /// slowest.
    #[NoAlpha]
    CIEDE2000
}

impl DistanceAlgorithm for EuclideanDistance {
    #[allow(clippy::eq_op, reason = "False positive")]
    fn distance(&self, left: &Rgba<u8>, right: &Rgba<u8>) -> u32 {
        let left = left.0.map(i32::from);
        let right = right.0.map(i32::from);

        ((left[0] - right[0]).pow(2)
            + (left[1] - right[1]).pow(2)
            + (left[2] - right[2]).pow(2)
            + (left[3] - right[3]).pow(2))
        .try_into()
        .unwrap()
    }
}

impl DistanceAlgorithm for ManhattanDistance {
    fn distance(&self, left: &Rgba<u8>, right: &Rgba<u8>) -> u32 {
        let left = left.0.map(i32::from);
        let right = right.0.map(i32::from);

        ((left[0] - right[0]).abs()
            + (left[1] - right[1]).abs()
            + (left[2] - right[2]).abs()
            + (left[3] - right[3]).abs())
        .try_into()
        .unwrap()
    }
}

impl DistanceAlgorithm for CIE76 {
    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        reason = "The cast should cause no issues here. If it does there is a bug further up."
    )]
    fn distance(&self, left: &Rgba<u8>, right: &Rgba<u8>) -> u32 {
        let left = Lab::from(left.to_rgb()).0;
        let right = Lab::from(right.to_rgb()).0;

        let val = (left[2] - right[2]).mul_add(
            left[2] - right[2],
            (left[1] - right[1]).mul_add(left[1] - right[1], (left[0] - right[0]).powi(2)),
        ) * 100_000.0;

        val as u32
    }
}

impl DistanceAlgorithm for CIEHybrid {
    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        reason = "The cast should cause no issues here. If it does there is a bug further up."
    )]
    fn distance(&self, left: &Rgba<u8>, right: &Rgba<u8>) -> u32 {
        let left = Lab::from(left.to_rgb()).0;
        let right = Lab::from(right.to_rgb()).0;

        let val = (left[2] - right[2])
            .mul_add(
                left[2] - right[2],
                (left[1] - right[1]).mul_add(left[1] - right[1], (left[0] - right[0]).powi(2)),
            )
            .mul_add(
                100_000.0,
                (left[0] - right[0]).abs()
                    + (left[1] - right[1]).abs()
                    + (left[2] - right[2]).abs(),
            );

        val as u32
    }
}

impl DistanceAlgorithm for CIEDE2000 {
    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        reason = "The cast should cause no issues here. If it does there is a bug further up."
    )]
    fn distance(&self, left: &Rgba<u8>, right: &Rgba<u8>) -> u32 {
        let l1 = Lab::from(left.to_rgb()).0;
        let l2 = Lab::from(right.to_rgb()).0;

        let (l1, a1, b1) = (f64::from(l1[0]), f64::from(l1[1]), f64::from(l1[2]));
        let (l2, a2, b2) = (f64::from(l2[0]), f64::from(l2[1]), f64::from(l2[2]));

        // Step 1: Compute C'ab and h'ab
        let c1 = a1.hypot(b1);
        let c2 = a2.hypot(b2);
        let c_avg = f64::midpoint(c1, c2);
        let c_avg7 = c_avg.powi(7);
        let g = 0.5 * (1.0 - (c_avg7 / (c_avg7 + 25_f64.powi(7))).sqrt());

        let a1p = a1 * (1.0 + g);
        let a2p = a2 * (1.0 + g);
        let c1p = a1p.hypot(b1);
        let c2p = a2p.hypot(b2);

        let h1p = b1.atan2(a1p).to_degrees().rem_euclid(360.0);
        let h2p = b2.atan2(a2p).to_degrees().rem_euclid(360.0);

        // Step 2: Compute delta L', delta C', delta H'
        let dl = l2 - l1;
        let dc = c2p - c1p;

        let dhp = if (c1p * c2p) == 0.0 {
            0.0
        } else if (h2p - h1p).abs() <= 180.0 {
            h2p - h1p
        } else if h2p - h1p > 180.0 {
            h2p - h1p - 360.0
        } else {
            h2p - h1p + 360.0
        };
        let dh = 2.0 * (c1p * c2p).sqrt() * (dhp / 2.0).to_radians().sin();

        // Step 3: CIEDE2000 weighting functions
        let l_avg = f64::midpoint(l1, l2);
        let c_avg_p = f64::midpoint(c1p, c2p);

        let h_avg_p = if (c1p * c2p) == 0.0 {
            h1p + h2p
        } else if (h1p - h2p).abs() <= 180.0 {
            f64::midpoint(h1p, h2p)
        } else if h1p + h2p < 360.0 {
            (h1p + h2p + 360.0) / 2.0
        } else {
            (h1p + h2p - 360.0) / 2.0
        };

        let t = 0.20f64.mul_add(
            -4.0f64.mul_add(h_avg_p, -63.0).to_radians().cos(),
            0.32f64.mul_add(
                3.0f64.mul_add(h_avg_p, 6.0).to_radians().cos(),
                0.24f64.mul_add(
                    (2.0 * h_avg_p).to_radians().cos(),
                    0.17f64.mul_add(-(h_avg_p - 30.0).to_radians().cos(), 1.0),
                ),
            ),
        );

        let sl = 1.0
            + 0.015 * (l_avg - 50.0).powi(2) / (l_avg - 50.0).mul_add(l_avg - 50.0, 20.0).sqrt();
        let sc = 0.045f64.mul_add(c_avg_p, 1.0);
        let sh = (0.015 * c_avg_p).mul_add(t, 1.0);

        let c_avg_p7 = c_avg_p.powi(7);
        let rc = 2.0 * (c_avg_p7 / (c_avg_p7 + 25_f64.powi(7))).sqrt();
        let d_theta = 30.0 * (-(((h_avg_p - 275.0) / 25.0).powi(2))).exp();
        let rt = -(rc * (2.0 * d_theta).to_radians().sin());

        let val = (rt * (dc / sc)).mul_add(
            dh / sh,
            (dh / sh).mul_add(dh / sh, (dc / sc).mul_add(dc / sc, (dl / sl).powi(2))),
        );

        (val * 1000.0) as u32
    }
}
