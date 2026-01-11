//! proc macros for pallete-mapper-* crates
//!
//! These macros are for internal use and not for end-consumers of the library or other components.
use std::fs::read_dir;

use heck::ToSnekCase;
use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{
    Attribute, Ident, Result,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

/// Represents a single algorithm declaration inside the macro input.
///
/// Example input fragment:
///
/// ```text
/// #[NoAlpha]
/// EuclideanDistanceNoAlpha
/// ```
struct Algorithm {
    /// Outer attributes attached to the identifier.
    ///
    /// In practice, this is primarily expected to contain `#[doc = "..."]`
    /// attributes generated from `///` comments.
    attrs: Vec<Attribute>,

    /// The identifier of the algorithm type.
    ident: Ident,

    /// If the algorithm respects alpha values
    alpha: bool,
}

/// Top-level macro input:
///
/// ```text
/// algorithms! {
///     /// Foo is a great algorithm
///     #[NoAlpha]
///     Foo
///     Bar
/// }
/// ```
struct AlgorithmsInput {
    /// The input algorithms
    algorithms: Vec<Algorithm>,
}

impl Parse for AlgorithmsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut algorithms = Vec::new();

        // Continue parsing until the macro input is exhausted.
        while !input.is_empty() {
            // Capture all outer attributes (e.g. doc comments).
            let mut attrs = input.call(Attribute::parse_outer)?;

            let alpha_pos = attrs
                .iter()
                .position(|attr| attr.path().is_ident("NoAlpha"));

            let alpha = alpha_pos.is_none_or(|alpha_pos| {
                attrs.remove(alpha_pos);
                false
            });

            // Parse the algorithm identifier.
            let ident: Ident = input.parse()?;

            algorithms.push(Algorithm {
                attrs,
                ident,
                alpha,
            });
        }

        Ok(Self { algorithms })
    }
}

/// The centeral *magic* enum for algorithms
///
/// ## What it does
///
/// 1. Let's you define an enum holding all algorithms defined
///
/// 2. Copies your provided doc comments to the structs generated
///
/// 3. Ensures that `DistanceAlgorithm` is actually implemented for all listed algorithms (the
///    compiler handles this for us via the trait bound on `algorithm()`)
///
/// 4. Appends doc comments regarding alpha values
///
/// 5. Generates tests
///
/// ## Syntax
///
/// ```text
/// algorithms! {
///     /// Foo is a great algorithm
///     #[NoAlpha]
///     Foo
///     Bar
/// }
/// ```
#[proc_macro]
pub fn algorithms(input: TokenStream) -> TokenStream {
    let AlgorithmsInput { algorithms } = parse_macro_input!(input as AlgorithmsInput);

    // Generate enum variants, duplicating the user's doc comments
    // and appending an additional "See: `StructName`" line.
    let enum_variants = algorithms.iter().map(|a| {
        let ident = &a.ident;
        let attrs = &a.attrs;
        let see_doc = format!("\n\nSee: [`{ident}`]");

        quote! {
            #(#attrs)*
            #[doc = #see_doc]
            #ident
        }
    });

    let algorithm_tests = algorithms.iter().fold(
        proc_macro2::TokenStream::new(),
        |mut token_stream, algorithm| {
            algorithm_tests(algorithm, &mut token_stream);

            token_stream
        },
    );

    // Generate match arms mapping enum variants to their
    // corresponding zero-sized struct implementations.
    let match_arms = algorithms.iter().map(|a| {
        let ident = &a.ident;
        quote! {
            Algorithms::#ident => #ident::default().distance(left, right)
        }
    });

    // Generate the concrete algorithm structs, preserving
    // the original doc comments verbatim.
    let structs = algorithms.iter().map(|a| {
        let ident = &a.ident;
        let attrs = &a.attrs;

        let alpha_doc = String::from(if a.alpha {
            "## Alpha Values\nThis algorithm **respects** alpha values.\nThis means `(255, 255, 255, 0) != (255, 255, 255, 255)`"
        } else {
            "## Alpha Values\nThis algorithm **does not** respects alpha values.\nThis means `(255, 255, 255, 0) == (255, 255, 255, 255)`"
        });

        quote! {
            #(#attrs)*
            #[doc = #alpha_doc]
            #[derive(Debug, Clone, Copy, Default)]
            pub struct #ident;
        }
    });

    // Final expansion.
    let expanded = quote! {
        #[cfg_attr(feature = "strum", derive(strum::EnumString, strum::Display, strum::EnumIter, strum::EnumCount, strum::VariantArray, strum::VariantNames))]
        /// Enumeration of all available distance algorithms.
        #[derive(Debug, Clone, Copy)]
        pub enum Algorithms {
            #( #enum_variants, )*
        }

        impl DistanceAlgorithm for Algorithms {
            fn distance(&self, left: &Rgba<u8>, right: &Rgba<u8>) -> u32 {
                match self {
                    #( #match_arms, )*
                }
            }
        }

        #( #structs )*

        #[cfg(test)]
        mod test {
            use crate::{
                Palette, color_pallete,
                map_image_to_palette,
                distance::*, // to get all algorithms
                rgba,
            };
            use image::{DynamicImage, ImageBuffer, ImageReader};
            use std::sync::LazyLock;
            use std::{io::Cursor, path::Path};

            static TESTING_PALLETE: LazyLock<Palette> = LazyLock::new(|| {
                color_pallete!(
                    [255, 0, 0, 255],     // Red
                    [255, 128, 0, 255],   // Orange
                    [255, 255, 0, 255],   // Yellow
                    [128, 255, 0, 255],   // Yellow-Green
                    [0, 255, 0, 255],     // Green
                    [0, 255, 128, 255],   // Spring Green
                    [0, 255, 255, 255],   // Cyan
                    [0, 128, 255, 255],   // Azure
                    [0, 0, 255, 255],     // Blue
                    [128, 0, 255, 255],   // Violet
                    [255, 0, 255, 255],   // Magenta
                    [255, 0, 128, 255],   // Rose
                    [255, 255, 255, 255], // White
                    [192, 192, 192, 255], // Light Gray
                    [64, 64, 64, 255],    // Dark Gray
                    [0, 0, 0, 255]        // Black
                )
            });


            #algorithm_tests
        }
    };

    expanded.into()
}

/// Creates the test for every algorithm
fn algorithm_tests(algorithm: &Algorithm, token_stream: &mut proc_macro2::TokenStream) {
    let algorithm_prefix = Ident::new(
        &algorithm.ident.to_string().to_snek_case(),
        algorithm.ident.span(),
    );

    let ident = &algorithm.ident;

    // Create test names based on the algorithm prefix and scenario
    let same_color_name = format_ident!("{}_same_color", algorithm_prefix);
    let white_black_name = format_ident!("{}_white_black", algorithm_prefix);
    let grey_name = format_ident!("{}_grey", algorithm_prefix);
    let alpha_name = format_ident!("{}_alpha", algorithm_prefix);
    let red_name = format_ident!("{}_red", algorithm_prefix);
    let green_name = format_ident!("{}_green", algorithm_prefix);
    let blue_name = format_ident!("{}_blue", algorithm_prefix);

    // Generate the test functions using quote! macro
    let tests = quote! {
        #[test]
        fn #same_color_name() {
            assert_eq!(
                #ident::default().distance(&rgba!(255, 255, 255), &rgba!(255, 255, 255)),
                0
            );
        }

        #[test]
        fn #white_black_name() {
            assert_ne!(
                #ident::default().distance(&rgba!(255, 255, 255), &rgba!(0, 0, 0)),
                0
            );
        }

        #[test]
        fn #grey_name() {
            assert!(
                #ident::default().distance(&rgba!(255, 255, 255), &rgba!(0, 0, 0))
                    > #ident::default().distance(&rgba!(255, 255, 255), &rgba!(127, 127, 127))
            );
        }

        #[test]
        fn #red_name() {
            assert_ne!(
                #ident::default().distance(&rgba!(255, 255, 255), &rgba!(0, 255, 255)),
                0
            );
        }

        #[test]
        fn #green_name() {
            assert_ne!(
                #ident::default().distance(&rgba!(255, 255, 255), &rgba!(255, 0, 255)),
                0
            );
        }

        #[test]
        fn #blue_name() {
            assert_ne!(
                #ident::default().distance(&rgba!(255, 255, 255), &rgba!(255, 255, 0)),
                0
            );
        }
    };

    tests.to_tokens(token_stream);

    if algorithm.alpha {
        let alpha = quote! {
            #[test]
            fn #alpha_name() {
                assert_ne!(
                    #ident::default().distance(&rgba!(255, 255, 255, 255), &rgba!(255, 255, 255, 0)),
                    0
                );
            }
        };

        alpha.to_tokens(token_stream);
    }

    // snapshot tests
    let dir = read_dir("assets/test-imgs/").expect("Failed to generate snapshot tests.");

    for entries in dir {
        let path = entries
            .expect("Failed to read snapshot input image.")
            .path();

        if let Some(file_name) = path.file_stem() {
            let test_name = format_ident!("{}_{}", algorithm_prefix, file_name.to_string_lossy());

            let img_path = format!("../../{}", path.to_string_lossy());

            let algorithm = &algorithm.ident;

            let snapshot_test = quote! {
                #[test]
                fn #test_name() {
                    let mut img = ImageReader::new(Cursor::new(include_bytes!(#img_path)));

                    img.set_format(image::ImageFormat::Png);

                    let mut img = img.decode().unwrap();

                    map_image_to_palette(&mut img, &TESTING_PALLETE, &#algorithm::default());
                    let mut buf = Vec::new();

                    let encoder = image::codecs::png::PngEncoder::new(&mut buf);

                    img.write_with_encoder(encoder).unwrap();

                    insta::assert_binary_snapshot!(".png", buf);
                }
            };

            snapshot_test.to_tokens(token_stream);
        }
    }
}
