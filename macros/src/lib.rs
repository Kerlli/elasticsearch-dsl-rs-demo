pub(crate) mod case;
mod helpers;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    Data,
    DataEnum,
    DeriveInput,
    Error,
    Variant,
};
use case::parse::{
    parse_case_attribute,
    parse_variant_attribute,
};

/// The `DisplayCase` macro automatically implements the `std::fmt::Display` trait
/// for enum types, using specified case formatting for each variant.
/// 
/// # Attributes
/// This macro supports the following attribute:
/// 
/// - `#[display_case(case = "snakecase")]`: Specifies the case conversion for the
/// enum variants. Supported values are `"lowercase"`, `"uppercase"`, and `"snakecase"`.
/// 
/// # Examples
/// 
/// ```
/// use macros::DisplayCase;
/// 
/// #[derive(DisplayCase)]
/// #[display_case(case = "snakecase")]
/// enum MyEnum {
///     FirstVariant,
///     #[display_case(display_as = "second")] // custom display value
///     SecondVariant,
/// }
/// 
/// assert_eq!(&MyEnum::FirstVariant.to_string(), "first_variant");
/// assert_eq!(&MyEnum::SecondVariant.to_string(), "second");
/// ```
#[proc_macro_derive(DisplayCase, attributes(display_case))]
pub fn derive_display_case(input: TokenStream) -> TokenStream {
    // Parse to syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let variants = match input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => return TokenStream::from(Error::new_spanned(&input, "DisplayCase can only be used with enums").to_compile_error())
    };

    let case = match parse_case_attribute(&input.attrs) {
        Ok(case) => case,
        Err(e) => return TokenStream::from(e.to_compile_error())
    };

    let pairs = match parse_variant_attribute(&variants) {
        Ok(pairs) => pairs,
        Err(e) => return TokenStream::from(e.to_compile_error())
    };

    let arms = variants.iter()
        .map(|Variant { ident, .. }| {
            match pairs.get(&ident) {
                Some(display_as) => quote! {
                    #name::#ident => write!(f, "{}", #display_as),
                },
                None => {
                    let display_val = case.parse_str(&ident.to_string());

                    quote! {
                        #name::#ident => write!(f, "{}", #display_val),
                    }
                }
            }
        });

    // Build the output
    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    #(#arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
