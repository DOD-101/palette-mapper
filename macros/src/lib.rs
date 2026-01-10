//! proc macros for pallete-mapper-* crates
//!
//! These macros are for internal use and not for end-consumers of the library or other components.
use proc_macro::TokenStream;
use quote::quote;
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
/// /// Euclidian docs
/// EuclidianDistance
/// ```
struct Algorithm {
    /// Outer attributes attached to the identifier.
    ///
    /// In practice, this is primarily expected to contain `#[doc = "..."]`
    /// attributes generated from `///` comments.
    attrs: Vec<Attribute>,

    /// The identifier of the algorithm type.
    ident: Ident,
}

/// Top-level macro input:
///
/// ```text
/// algorithms! {
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
            let attrs = input.call(Attribute::parse_outer)?;

            // Parse the algorithm identifier.
            let ident: Ident = input.parse()?;

            algorithms.push(Algorithm { attrs, ident });
        }

        Ok(Self { algorithms })
    }
}

/// Generates the enum holding all algorithms and the individual algorithm structs
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

    // Generate match arms mapping enum variants to their
    // corresponding zero-sized struct implementations.
    let match_arms = algorithms.iter().map(|a| {
        let ident = &a.ident;
        quote! {
            Algorithms::#ident => #ident
        }
    });

    // Generate the concrete algorithm structs, preserving
    // the original doc comments verbatim.
    let structs = algorithms.iter().map(|a| {
        let ident = &a.ident;
        let attrs = &a.attrs;

        quote! {
            #(#attrs)*
            pub struct #ident;
        }
    });

    // Final expansion.
    let expanded = quote! {
        #[cfg_attr(feature = "strum", derive(strum::EnumString, strum::Display, strum::EnumIter, strum::EnumCount, strum::VariantArray, strum::VariantNames))]
        /// Enumeration of all available distance algorithms.
        pub enum Algorithms {
            #( #enum_variants, )*
        }

        impl Algorithms {
            /// Returns the concrete algorithm implementation
            /// corresponding to this enum variant.
            pub fn algorithm(&self) -> impl DistanceAlgorithm {
                match self {
                    #( #match_arms, )*
                }
            }
        }

        #( #structs )*
    };

    expanded.into()
}
