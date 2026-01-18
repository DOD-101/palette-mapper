//! Wasm code to allow using [`palette_mapper`] as a local web tool
use std::io::Cursor;
use std::str::FromStr;
use std::sync::LazyLock;

use image::ImageReader;
use palette_mapper::distance::Algorithms;
use palette_mapper::{Palette, color_pallete, map_image_to_palette};
use strum::VariantNames;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn algorithms() -> Vec<String> {
    Algorithms::VARIANTS
        .iter()
        .map(std::string::ToString::to_string)
        .collect()
}

#[wasm_bindgen]
pub enum MapErr {
    InvalidAlgorithm,
    InavlidImg,
    ConversionFailed,
    FormatNotUnderstood,
    FailedToEncode,
}

static TESTING_PALLETE: LazyLock<Palette> = LazyLock::new(|| {
    color_pallete!(
        [24, 24, 37, 255],
        [30, 30, 46, 255],
        [17, 17, 27, 255],
        [205, 214, 244, 255],
        [166, 173, 200, 255],
        [186, 194, 222, 255],
        [108, 112, 134, 255],
        [127, 132, 156, 255],
        [147, 153, 178, 255],
        [88, 91, 112, 255],
        [69, 71, 90, 255],
        [49, 50, 68, 255],
        [243, 139, 168, 255],
        [235, 160, 172, 255],
        [250, 179, 135, 255],
        [249, 226, 175, 255],
        [166, 227, 161, 255],
        [148, 226, 213, 255],
        [137, 220, 235, 255],
        [116, 199, 236, 255],
        [137, 180, 250, 255],
        [180, 190, 254, 255],
        [203, 166, 247, 255],
        [245, 194, 231, 255],
        [242, 205, 205, 255],
        [245, 224, 220, 255]
    )
});

#[wasm_bindgen]
pub fn map_image(img: Vec<u8>, algorithm: String) -> Result<Vec<u8>, MapErr> {
    let mut output = Cursor::new(Vec::with_capacity(img.len()));

    let reader = ImageReader::new(Cursor::new(img))
        .with_guessed_format()
        .map_err(|_| MapErr::FormatNotUnderstood)?;

    let format = reader.format().unwrap();

    let mut buf = reader.decode().map_err(|_| MapErr::InavlidImg)?;

    map_image_to_palette(
        &mut buf,
        &TESTING_PALLETE,
        &Algorithms::from_str(&algorithm).map_err(|_| MapErr::InvalidAlgorithm)?,
    );

    buf.write_to(&mut output, format)
        .map_err(|_| MapErr::FailedToEncode)?;

    Ok(output.into_inner())
}
