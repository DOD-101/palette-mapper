//! Library to convert (map) an image to color pallete
use image::Rgba;

#[cfg(feature = "rayon")]
use image::DynamicImage;

#[cfg(not(feature = "rayon"))]
use image::{GenericImage, GenericImageView};

pub mod distance;
#[macro_use]
pub mod palette;
mod conversions;

pub use {distance::Distance, palette::Palette};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// Take a color and find the closest color to it in a pallete
///
/// [`Rgba`]: image::Rgba
#[must_use]
pub fn closest_color_in_pallete<'b, D: distance::DistanceAlgorithm>(
    color: &Rgba<u8>,
    palette: &'b palette::Palette,
    algorithm: &D,
) -> Option<&'b Rgba<u8>> {
    let mut min: distance::Distance<D> = distance::Distance::new_max();
    let mut col = None;

    for pcolor in palette {
        let dist = distance::Distance::new(color, pcolor, algorithm);

        if dist < min {
            min = dist;

            col = Some(pcolor);
        }
    }

    col
}

/// Take an image and convert it to a color pallete  
///
/// ## Panics
///
/// This function panics if `palette` doesn't contain any colors.
pub fn map_image_to_palette<D: distance::DistanceAlgorithm + Sync>(
    img: &mut image::DynamicImage,
    palette: &palette::Palette,
    algorithm: &D,
) {
    map_image_to_palette_inner(img, palette, algorithm);
}

#[cfg(not(feature = "rayon"))]
/// Inner sequential implementation of [`map_image_to_palette`]
///
/// ## Panics
///
/// See [`map_image_to_palette`]
fn map_image_to_palette_inner<D: distance::DistanceAlgorithm>(
    img: &mut image::DynamicImage,
    palette: &palette::Palette,
    algorithm: &D,
) {
    let width = img.width();
    let height = img.height();

    for x in 0..width {
        for y in 0..height {
            let px = img.get_pixel(x, y);
            let col = closest_color_in_pallete(&px, palette, algorithm);

            img.put_pixel(x, y, *col.unwrap());
        }
    }
}

#[cfg(feature = "rayon")]
/// Inner parallel implementation of [`map_image_to_palette`]
///
/// ## Panics
///
/// See [`map_image_to_palette`]
fn map_image_to_palette_inner<D: distance::DistanceAlgorithm + Sync>(
    img: &mut image::DynamicImage,
    palette: &palette::Palette,
    algorithm: &D,
) {
    match img {
        DynamicImage::ImageRgb8(buf) => {
            buf.par_enumerate_pixels_mut().for_each(|px| {
                let px = px.2;

                let pixel = image::Rgba([px[0], px[1], px[2], 255]);
                let col = closest_color_in_pallete(&pixel, palette, algorithm).unwrap();
                *px = [col[0], col[1], col[2]].into();
            });
        }

        DynamicImage::ImageRgba8(buf) => {
            buf.par_enumerate_pixels_mut().for_each(|px| {
                let px = px.2;

                let pixel = image::Rgba([px[0], px[1], px[2], px[3]]);
                let col = closest_color_in_pallete(&pixel, palette, algorithm).unwrap();
                *px = *col;
            });
        }
        // fallback
        d => {
            let buf = d.clone().into_rgba8();

            map_image_to_palette_inner(&mut DynamicImage::from(buf), palette, algorithm);
        }
    }
}
