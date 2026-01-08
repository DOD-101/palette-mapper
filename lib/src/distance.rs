//! Items relating to determining the "distance" between two colors
//!
//! ## What is a "distance" in this context?
//!
//! The distance between colors can be understood as how "similar" colors are.
//! If the distance is very small the color are very similar. If it is large they are very
//! different.
//!
//! Example:
//!     #ffffff (white) and #eeeeee are more similar than #ffffff and #000000, so their distance
//!     is less.
//!
//! ## Determining the distance between colors
//!
//! There are many different approaches to determining the "distance" between colors.
//! For this purpose the [`DistanceAlgorithm`] trait exists. Any type implementing this trait ca be
//! used to calculate the distance between two colors.
//!
//! See the implementors  of [`DistanceAlgorithm`] for different ways of calculating the distance.
//!
//! ### See also
//!
//! - [`Distance`]
use std::marker::PhantomData;

use image::Rgba;

/// A distance between two colors
///
/// See [module level docs](`self`)
#[derive(Default, Debug)]
pub struct Distance<A: DistanceAlgorithm = EuclidianDistance> {
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
    pub fn new(left: &Rgba<u8>, right: &Rgba<u8>) -> Self {
        Self {
            distance: A::distance(left, right),
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

/// Trait representing an algorithm used to calculate the distance between two colors
///
/// ## Implementing this trait
///
/// When implementing this trait it is not relevant what the concrete values returned by
/// [`DistanceAlgorithm::distance`] are. They are never exposed to the user directly. They must
/// merely be a consistent measrement of how close two colors are to one another.  
///
/// This means one Algorithm may return values in the range `0-100` while another uses the entire
/// range of [`u32`] values. As long as the values returned allow for comparing how close (or
/// similar) two colors they are both valid.
pub trait DistanceAlgorithm {
    /// Function used to determine the distance of two colors
    fn distance(left: &Rgba<u8>, right: &Rgba<u8>) -> u32;
}

/// Calculation of the distance of two colors using the [Euclidean distance](https://en.wikipedia.org/wiki/Euclidean_distance)
///
/// This Algorithm respects the alpha value.
pub struct EuclidianDistance;

impl DistanceAlgorithm for EuclidianDistance {
    #[allow(clippy::eq_op, reason = "False positive")]
    fn distance(left: &Rgba<u8>, right: &Rgba<u8>) -> u32 {
        let left = left.0.map(i32::from);
        let right = right.0.map(i32::from);

        ((left[0] - right[0]).pow(2)
            + (left[1] - right[1]).pow(2)
            + (left[2] - left[2]).pow(2)
            + (left[3] - left[3]).pow(2))
        .try_into()
        .unwrap()
    }
}
