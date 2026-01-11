//! CLI tool for `palette_mapper` lib
//!
//! ## Currently supported formats for pallete
//!
//! The used palette is read from a file. Currently supported formats for this file are:
//!
//! - json
//!
//! ## Usage
//!
//! `palette-mapper ./input.png palette.json`
//!
//! For more options run `palette-mapper --help`
use anyhow::{Ok, Result, anyhow, bail};
use clap::{
    Parser,
    builder::{PossibleValuesParser, TypedValueParser},
};
use image::DynamicImage;
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use palette_mapper::{Palette, distance::Algorithms, map_image_to_palette};

/// CLI struct containing options passed by user
#[derive(Parser)]
#[clap(about = "Convert an image to a color palette")]
struct Cli {
    /// Path to input image
    input: PathBuf,
    /// Path to file containing palette
    // TODO: Allow passing values by stdin
    palette: PathBuf,
    /// Distance Algorithm used to determine distance between colors
    #[arg(long, short, value_enum,
        value_parser = PossibleValuesParser::new(<Algorithms as strum::VariantNames>::VARIANTS).map(|s| s.parse::<Algorithms>().unwrap()),
        default_value = "EuclideanDistance")]
    algorithm: Algorithms,
    /// Output path
    ///
    /// Having the path end with ".{ext}" will replace the extension with that of the input file.
    #[arg(long, short, default_value = "output.{ext}")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let palette = get_palette(cli.palette)?;

    let mut img = open_image(&cli.input)?;

    let algorithm = &cli.algorithm;

    map_image_to_palette(&mut img, &palette, algorithm);

    let mut output_path = cli.output;

    #[allow(clippy::literal_string_with_formatting_args, reason = "False positive")]
    if output_path.extension().is_some_and(|ext| ext == "{ext}") {
        if let Some(input_ext) = cli.input.extension() {
            output_path.set_extension(input_ext);
        } else {
            output_path.set_extension("");
        }
    }

    img.save(output_path)
        .map_err(|_| anyhow!("unsupported output format"))?;

    Ok(())
}

/// Attempt to read the provided path and deserialize the contents to a [`Palette`]
///
/// Currently only supports json.
fn get_palette(palette: PathBuf) -> Result<Palette> {
    let format = palette.extension().map_or_else(
        || {
            eprintln!("No extension on palette path. Assuming line-wise.");

            "line-wise".to_string()
        },
        |v| v.to_string_lossy().to_string(),
    );

    match format.as_str() {
        "json" => {
            let file = File::open(palette)?;

            let buffered = BufReader::new(file);

            Ok(serde_json::from_reader(buffered)?)
        }
        _ => bail!("Unsupported format for palette. Supported formats are: json"),
    }
}

/// Opens the input image at the given path
fn open_image<P>(path: P) -> Result<DynamicImage>
where
    P: AsRef<Path>,
{
    Ok(image::ImageReader::open(path)
        .map_err(|_| anyhow!("could not open input image"))?
        .with_guessed_format()
        .map_err(|_| anyhow!("could not determine input image format"))?
        .decode()
        .expect("Format must have been determined at this point!"))
}
