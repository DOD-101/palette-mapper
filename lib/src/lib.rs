//! Library to convert (map) an image to color pallete
use image::{GenericImage, GenericImageView, Rgba};

pub mod distance;
#[macro_use]
pub mod palette;

pub use {distance::Distance, palette::Palette};

/// Take a color and find the closest color to it in a pallete
///
/// [`Rgba`]: image::Rgba
#[must_use]
pub fn closest_color_in_pallete<'b, D: distance::DistanceAlgorithm>(
    color: &Rgba<u8>,
    palette: &'b palette::Palette,
) -> Option<&'b Rgba<u8>> {
    let mut min: distance::Distance<D> = distance::Distance::new_max();
    let mut col = None;

    for pcolor in palette {
        let dist = distance::Distance::new(color, pcolor);

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
pub fn map_image_to_palette<D: distance::DistanceAlgorithm>(
    img: &mut image::DynamicImage,
    palette: &palette::Palette,
) {
    let width = img.width();
    let height = img.height();

    for x in 0..width {
        for y in 0..height {
            let px = img.get_pixel(x, y);
            let col = closest_color_in_pallete::<D>(&px, palette);

            img.put_pixel(x, y, *col.unwrap());
        }
    }
}
