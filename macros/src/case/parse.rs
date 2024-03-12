use std::collections::HashMap;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Comma, Eq},
    Attribute,
    Error,
    Ident,
    LitStr,
    Variant,
};
use crate::case::Case;

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

pub fn parse_case_attribute(attrs: &[Attribute]) -> syn::Result<Case> {
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

struct DisplayCaseVariantAttr {
    value: LitStr,
}

impl Parse for DisplayCaseVariantAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let as_ident: Ident = input.parse()?;
        if as_ident != "display_as" {
            return Err(syn::Error::new(as_ident.span(), "Expected 'display_as'"));
        }
        let _: Eq = input.parse()?;
        let value: LitStr = input.parse()?;
        Ok(DisplayCaseVariantAttr { value })
    }
}

pub fn parse_variant_attribute(variants: &Punctuated<Variant, Comma>) -> syn::Result<HashMap<Ident, String>> {
    let mut pairs: HashMap<Ident, String> = HashMap::new();

    for variant in variants.iter().filter(|variant| !variant.attrs.is_empty()) {
        for attr in &variant.attrs {
            if attr.path().is_ident("display_case") {
                let parsed_attr = attr.parse_args::<DisplayCaseVariantAttr>()?;
                pairs.insert(variant.ident.clone(), parsed_attr.value.value());
            }
        }
    }

    Ok(pairs)
}
