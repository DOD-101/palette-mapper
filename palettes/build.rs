//! Build script for generating Palettes from base16/24 themes
//!
//! ## How it works
//!
//! 1. We run `fetch-palettes.sh` in the git repos root, to update the vendored theme files
//!
//! 2. This file reads those files during the build and creates the `lib.rs` file. This is done
//!    using [`askama`] as a templating system.
//!
//! 3. The library can be used like any other. Containing a `Base16` and `Base24` enum with all of
//!    themes, which can be converted into [`palette_mapper::Palette`]s simply by using `.into()`
use askama::Template;
use palette_mapper::Palette;

use std::{
    env,
    fs::{File, read_dir, write},
    io::Read,
    path::Path,
};

/// Load the theme files found at the dir `path` and convert them into a `Vec<Theme>`
fn get_themes(path: impl AsRef<Path>) -> Vec<Theme> {
    read_dir(path)
        .unwrap()
        .map(|v| {
            let entry = v.unwrap();

            let name = entry
                .file_name()
                .to_string_lossy()
                .to_string()
                .strip_suffix(".json")
                .unwrap()
                .to_string();

            let mut content = String::new();

            File::open(entry.path())
                .unwrap()
                .read_to_string(&mut content)
                .expect("Should not fail to read theme file.");

            Theme {
                name,
                palette: serde_json::from_str(&content).unwrap(),
            }
        })
        .collect()
}

fn main() {
    let l = Lib {
        base16: get_themes("./base16/"),
        base24: get_themes("./base24/"),
    };

    let out_dir = env::var("OUT_DIR").unwrap();

    println!("HERE: {out_dir}");
    println!("HERE: {out_dir}");

    let content = l.render().unwrap();
    write(out_dir + "/generated.rs", content).unwrap();
}

#[allow(clippy::unnecessary_wraps, reason = "Result needed by askama macro")]
mod filters {
    //! Custom filters for askama templates
    use heck::ToUpperCamelCase;
    use std::fmt::Display;

    #[askama::filter_fn]
    pub fn enum_name<'a>(inp: &'a dyn Display, _: &dyn askama::Values) -> askama::Result<String> {
        let mut out = inp.to_string().to_upper_camel_case();

        if out.starts_with(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].as_slice()) {
            out.insert_str(0, "Theme");
        }

        Ok(out)
    }
}

#[derive(Template)]
#[template(path = "lib.rs.askama", escape = "none")]
/// Struct representing all data needed to construct the main lib file
struct Lib {
    /// The base 16 themes
    base16: Vec<Theme>,
    /// The base 24 themes
    base24: Vec<Theme>,
}

/// A single theme found in the lib
///
/// Themes are sourced form the `./base{16,24}/*.json` files.
struct Theme {
    /// Name of the theme, derived from the file name
    name: String,
    /// Palette of the theme
    palette: Palette,
}
