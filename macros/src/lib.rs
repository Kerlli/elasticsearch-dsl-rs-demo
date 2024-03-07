mod case;
mod helpers;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Eq,
    Attribute,
    Data,
    DataEnum,
    DeriveInput,
    Error,
    Ident,
    LitStr,
    Variant,
};
use case::Case;

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
///     SecondVariant,
/// }
/// ```
/// 
/// Using the above definition, `MyEnum::FirstVariant` will be displayed as "first_variant".
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

    let arms = variants.iter().map(|Variant {ident, ..}| {
        let variant_name = case.parse_str(&ident.to_string());

        quote! {
            #name::#ident => write!(f, "{}", #variant_name),
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

struct DisplayCaseAttr {
    value: LitStr,
}

impl Parse for DisplayCaseAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let case_ident: Ident = input.parse()?;
        if case_ident != "case" {
            return Err(syn::Error::new(case_ident.span(), "Expected 'case'"));
        }
        let _: Eq = input.parse()?;
        let value: LitStr = input.parse()?;
        Ok(DisplayCaseAttr { value })
    }
}

fn parse_case_attribute(attrs: &[Attribute]) -> syn::Result<Case> {
    for attr in attrs {
        if attr.path().is_ident("display_case") {
            let parsed_attr = attr.parse_args::<DisplayCaseAttr>()?;
            let case_str = parsed_attr.value.value();
            return case_str.parse::<Case>()
                .map_err(|err| Error::new_spanned(&parsed_attr.value, err));
        }
    }
    Err(Error::new_spanned(&attrs[0], "Attribute `display_case` not found on enum"))
}


