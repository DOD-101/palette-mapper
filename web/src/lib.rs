//! Wasm code to allow using [`palette_mapper`] as a local web tool
use std::io::Cursor;
use std::str::FromStr;

use image::{ImageReader, Rgba};
use palette_mapper::distance::Algorithms;
use palette_mapper::{Palette, map_image_to_palette};
use strum::VariantNames;

use wasm_bindgen::prelude::*;

/// Function to return a list of all types of algorithms
#[wasm_bindgen]
pub fn algorithms() -> Vec<String> {
    Algorithms::VARIANTS
        .iter()
        .map(std::string::ToString::to_string)
        .collect()
}

/// Errors encountered when attempting to map an image to a color palette
///
/// These are all errors specific to [`map_image`], since it is responsible for interfacing with
/// the web frontend. Any errors are to be treated as bugs. We could just panic, but this gives us
/// an easier method of working with the errors. This might change in the future
#[wasm_bindgen]
pub enum MapErr {
    /// The passed algorithm [`&str`] cannot be converted into an algorithm
    InvalidAlgorithm,
    /// The passed bytes cannot be interpreted as an image
    InvalidImg,
    /// The format of the image could not be determined
    FormatNotUnderstood,
    /// Failed to (re-)encode the image after conversion and write it to the output buffer
    FailedToEncode,
    /// The passed rgba values in the palette are invalid
    InvalidRgbaValues,
}

/// Main function used for interfacing with the js code to facilitate the conversion of images
///
/// This function takes both in very simple types and then converts them internally into the proper
/// types to be passed to [`map_image_to_palette`], returning errors along the way. If used
/// correctly this function **should** not error. This is not a guarantee.
///
/// ## Errors
///
/// See: [`MapErr`]
///
/// ## Panics
///
/// This function should never panic, instead error-ing as necessary. This might change in the
/// future.
#[wasm_bindgen]
pub fn map_image(img: Vec<u8>, palette: &[u8], algorithm: &str) -> Result<Vec<u8>, MapErr> {
    let mut output = Cursor::new(Vec::with_capacity(img.len()));

    let reader = ImageReader::new(Cursor::new(img))
        .with_guessed_format()
        .map_err(|_| MapErr::FormatNotUnderstood)?;

    let format = reader
        .format()
        .expect("Since we called with guessed format the format must be known");

    let mut buf = reader.decode().map_err(|_| MapErr::InvalidImg)?;

    let mut pal = Palette::default();

    let chunks = {
        let (chunks, remainder) = palette.as_chunks::<4>();

        if !remainder.is_empty() {
            return Err(MapErr::InvalidRgbaValues);
        }

        chunks
    };

    for cols in chunks {
        pal.add_color(Rgba::from([cols[0], cols[1], cols[2], cols[3]]));
    }

    map_image_to_palette(
        &mut buf,
        &pal,
        &Algorithms::from_str(algorithm).map_err(|_| MapErr::InvalidAlgorithm)?,
    );

    buf.write_to(&mut output, format)
        .map_err(|_| MapErr::FailedToEncode)?;

    Ok(output.into_inner())
}
