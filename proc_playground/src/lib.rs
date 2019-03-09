extern crate proc_macro;
use proc_macro::TokenStream;

use quote::{quote, quote_spanned, ToTokens};
use syn::{AttributeArgs, parse_macro_input, DeriveInput, Data, Fields, Generics, GenericParam, parse_quote, Index, Lit, LitStr, Ident, Variant, ItemStruct, FieldsNamed, ItemEnum};
use syn::spanned::Spanned;
use proc_helper_playground::*;


macro_rules! add_fields_helper {
    ($name:ident, $($fields:tt)*) => {
        #[proc_macro_attribute]
        pub fn $name (attr: TokenStream, item: TokenStream) -> TokenStream {
            let attr = parse_macro_input!(attr as AttributeArgs);
            let item = parse_macro_input!(item as ItemStruct);

            let new_fields = quote!($($fields)*);
            // return syn::Error::new(item.span(), format!("{:#?}", new_fields)).to_compile_error().into();
            // TokenStream::new()
            backend_add_fields(&attr, &item, &new_fields)
                .unwrap_or_else(|error| proc_macro2::TokenStream::from(error.to_compile_error()))
                .into()
        }
    };
}


#[proc_macro]
pub fn macro_caller(item: TokenStream) -> TokenStream {
    let macro_ident = parse_macro_input!(item as Ident);
    quote!(#macro_ident!())
        .into()
}

#[proc_macro]
pub fn bidirectional_enum(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as BidirectionalEnum)
        .into_token_stream()
        .into()
}


#[proc_macro]
pub fn stupid_expander(_item: TokenStream) -> TokenStream {
    "#[derive(Debug, Derp)]
        pub enum SomeDerpStructDeriver {
            #[gribothy_variant_value = \"123u8\"]
            First,
            #[gribothy_variant_value = \"8u8..=22u8 | 43u8..=87u8\"]
            Second(u8),
        }"
        .parse().unwrap()
}

// #[proc_macro]
// pub fn no_way_expand(item: TokenStream) -> TokenStream {
//     "#[derive(Debug)]".parse().unwrap()
// }


#[proc_macro_attribute]
pub fn enum_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as AttributeArgs);
    let item_clone = item.clone();
    let enum_item = parse_macro_input!(item_clone as ItemEnum);
    match backend_enum_type(&attr, &enum_item) {
        Ok(()) => item,
        Err(error) => proc_macro2::TokenStream::from(error.to_compile_error()).into(),
    }

}


#[proc_macro_derive(Derp, attributes(gribothy_variant_value))]
pub fn derive_derp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(backend_derive_derp(&input))
}


add_fields_helper!(add_fields, 
    pub field_extra: u8,
    pub second_extra: u32,
);





// #[proc_macro_attribute]
// pub fn add_fields(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let attr = parse_macro_input!(attr as AttributeArgs);
//     let item = parse_macro_input!(item as ItemStruct);
//     // TokenStream::new()
//     backend_add_fields(&attr, &item)
//         .unwrap_or_else(|error| proc_macro2::TokenStream::from(error.to_compile_error()))
//         .into()
//         // .map(TokenStream::from)
//         // .as_ref()
//         // .map_err(syn::Error::to_compile_error)
//         // .unwrap_or_else(syn::Error::to_compile_error)
    
// } 