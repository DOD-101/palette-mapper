//! Wasm code to allow using [`palette_mapper`] as a local web tool
use std::io::Cursor;
use std::str::FromStr;

use image::ImageReader;
use palette_mapper::distance::Algorithms;
use palette_mapper::{Palette, map_image_to_palette};
use palette_mapper_palettes::{Base16, Base24};
use strum::IntoEnumIterator;

use wasm_bindgen::prelude::*;

/// Return a list of all algorithms
#[wasm_bindgen]
#[must_use]
pub fn algorithms() -> Vec<String> {
    Algorithms::iter().map(|v| v.to_string()).collect()
}

// TODO: Make this DRY (macro)

/// Get all base24 themes
#[wasm_bindgen]
#[must_use]
pub fn base24() -> Vec<String> {
    Base24::iter().map(|v| v.to_string()).collect()
}

/// Get all base16 themes
#[wasm_bindgen]
#[must_use]
pub fn base16() -> Vec<String> {
    Base16::iter().map(|v| v.to_string()).collect()
}

/// Try to parse `theme` to base16 theme
#[wasm_bindgen]
#[cfg(target_family = "wasm")]
pub fn from_base_16_name(theme: &str) -> Result<Base16, MapErr> {
    Base16::from_str(theme).map_err(|_| MapErr::InvalidThemeString)
}

/// Try to parse `theme` to base24 theme
#[wasm_bindgen]
#[cfg(target_family = "wasm")]
pub fn from_base_24_name(theme: &str) -> Result<Base24, MapErr> {
    Base24::from_str(theme).map_err(|_| MapErr::InvalidThemeString)
}

/// Return palette of base16 theme
#[wasm_bindgen]
#[cfg(target_family = "wasm")]
pub fn pal_from_base16(base: Base16) -> String {
    serde_json::to_string(&Palette::from(base)).unwrap()
}

/// Return palette of base24 theme
#[wasm_bindgen]
#[cfg(target_family = "wasm")]
pub fn pal_from_base24(base: Base24) -> String {
    serde_json::to_string(&Palette::from(base)).unwrap()
}

/// Errors encountered when attempting to map an image to a color palette
///
/// These are all errors specific to [`map_image`], since it is responsible for interfacing with
/// the web frontend. Any errors are to be treated as bugs. We could just panic, but this gives us
/// an easier method of working with the errors. This might change in the future
#[wasm_bindgen]
#[derive(Debug)]
pub enum MapErr {
    /// The passed algorithm [`&str`] cannot be converted into an algorithm
    InvalidAlgorithm,
    /// The passed bytes cannot be interpreted as an image
    InvalidImg,
    /// The format of the image could not be determined
    FormatNotUnderstood,
    /// Failed to (re-)encode the image after conversion and write it to the output buffer
    FailedToEncode,
    /// The passed string for the palette could not be deserialized successfully
    InvalidPaletteString,

    /// The passed string could not be converted to a theme
    InvalidThemeString,
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
pub fn map_image(img: Vec<u8>, palette: &str, algorithm: &str) -> Result<Vec<u8>, MapErr> {
    let mut output = Cursor::new(Vec::with_capacity(img.len()));

    let reader = ImageReader::new(Cursor::new(img))
        .with_guessed_format()
        .map_err(|_| MapErr::FormatNotUnderstood)?;

    let format = reader
        .format()
        .expect("Since we called with guessed format the format must be known");

    let mut buf = reader.decode().map_err(|_| MapErr::InvalidImg)?;

    let pal: Palette = serde_json::from_str(palette).map_err(|_| MapErr::InvalidPaletteString)?;

    map_image_to_palette(
        &mut buf,
        &pal,
        &Algorithms::from_str(algorithm).map_err(|_| MapErr::InvalidAlgorithm)?,
    );

    buf.write_to(&mut output, format)
        .map_err(|_| MapErr::FailedToEncode)?;

    Ok(output.into_inner())
}
