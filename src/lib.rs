//! A simple derive macro to generate an `ordinal()`
//! method for enums.
//!
//! Unlike `num_derive::ToPrimitive`, this derive macro
//! allows non-C-like enums. The `ordinal` function reflects
//! the variant of the enum and does not account
//! for fields.
//!
//! # Example
//! ```
//! use ordinalizer::Ordinal;
//! #[derive(Ordinal)]
//! enum Animal {
//!     Dog,
//!     Cat {
//!         age: i32,
//!     }
//! }
//!
//! assert_eq!(Animal::Dog.ordinal(), 0);
//! assert_eq!((Animal::Cat { age: 10 }).ordinal(), 1);
//! ```

use proc_macro2::{Ident, TokenStream};
use proc_macro_error::*;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

struct Variant<'a> {
    ident: &'a Ident,
    unit_field_count: usize,
    has_named_fields: bool,
}

/// Generates a `fn ordinal(&self) -> usize` for an enum.
///
/// The enum may have any number of variants. It is not
/// required to be a C-like enum, i.e. its variants
/// may have named or unnamed fields.
///
/// The returned ordinals will correspond to the variant's
/// index in the enum definition. For example, the first
/// variant of enum will have ordinal `0`.
#[proc_macro_error]
#[proc_macro_derive(Ordinal)]
pub fn derive_ordinal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let variants = detect_variants(&input);

    let match_arms = generate_match_arms(&variants, &input);

    let enum_ident = &input.ident;

    let tokens = quote! {
        impl #enum_ident {
            pub fn ordinal(&self) -> usize {
                match self {
                    #(#match_arms,)*
                }
            }
        }
    };
    tokens.into()
}

fn detect_variants(input: &DeriveInput) -> Vec<Variant> {
    let mut vec = Vec::new();

    let data = match &input.data {
        syn::Data::Enum(data) => data,
        _ => abort_call_site!("cannot derive `Ordinal` on an item which is not an enum"),
    };

    for variant in &data.variants {
        vec.push(detect_variant(variant));
    }

    vec
}

fn detect_variant(variant: &syn::Variant) -> Variant {
    let ident = &variant.ident;

    let (unit_field_count, has_named_fields) = match &variant.fields {
        syn::Fields::Named(_) => (0, true),
        syn::Fields::Unit => (0, false),
        syn::Fields::Unnamed(unnanmed) => (unnanmed.unnamed.len(), false),
    };

    Variant {
        ident,
        unit_field_count,
        has_named_fields,
    }
}

fn generate_match_arms(variants: &[Variant], input: &DeriveInput) -> Vec<TokenStream> {
    let mut vec = Vec::new();
    let enum_ident = &input.ident;

    for (ordinal, variant) in variants.iter().enumerate() {
        let variant_ident = variant.ident;
        let pattern = match (variant.has_named_fields, variant.unit_field_count) {
            (true, _) => quote! { #enum_ident::#variant_ident { .. } },
            (false, x) if x != 0 => {
                let underscores: Vec<_> = (0..x).map(|_| quote! { _ }).collect();

                quote! {
                    #enum_ident::#variant_ident(#(#underscores),*)
                }
            }
            (false, 0) => quote! { #enum_ident::#variant_ident },
            _ => unreachable!(),
        };

        vec.push(quote! {
            #pattern => #ordinal
        });
    }

    vec
}
