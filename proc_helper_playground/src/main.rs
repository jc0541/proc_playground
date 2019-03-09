use quote::{quote, quote_spanned};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Generics, GenericParam, parse_quote, Index, Lit, LitStr, Variant};
use syn::spanned::Spanned;

use proc_macro2::TokenStream;

use proc_helper_playground::*;

// #[derive(Debug, Derp)]
//         pub struct SomeDerpStructDeriver {
//             #[derpy = 123u8]
//             pub byte: u8,
//             #[derpy = 8u8..=22u8 | 43u8..=8u8]
//             pub unattred: u16,
//         }
    

fn attribute_macro() {
    // let raw = "
    //     #[add_fields(df)]
    //     pub struct Dingus {
    //         pub initial_field: bool
    //     }
    // ";
    // let input = syn::parse_str::<ItemStruct>(Deri)
}




fn bidirection_enumer() {
    let raw = r#"
        pub enum NoWay : u8 {
            Derply <=> 0u8..=8u8 | 9u8..=11u8,
            Stupid <=> 111u8,
        }
    "#;

    let what = syn::parse_str::<BidirectionalEnum>(raw);

    let huh = 1;
}




fn derive_macro() {

    let raw = "
        #[derive(Debug, Derp)]
        pub enum SomeDerpStructDeriver {
            #[gribothy_variant_value = \"123u8\"]
            First,
            #[gribothy_variant_value = \"8u8..=22u8 | 43u8..=87u8\"]
            Second(u8),
        }
    ";
    let input = syn::parse_str::<DeriveInput>(raw).unwrap();

    let input_str = format!("{:#?}", input.data);
    println!("{}", input_str);

    let huh = 1;

}   


fn main() {
    bidirection_enumer();
    
}




// "Struct(DataStruct { struct_token: Struct, fields: Named(FieldsNamed { brace_token: Brace, named: [Field { attrs: [Attribute { pound_token: Pound, style: Outer, bracket_token: Bracket, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident(derpy), arguments: None }] }, tts: TokenStream [Punct { op: '=', spacing: Alone }, Literal { lit: 123u8 }] }], vis: Public(VisPublic { pub_token: Pub }), ident: Some(Ident(byte)), colon_token: Some(Colon), ty: Path(TypePath { qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident(u8), arguments: None }] } }) }, Comma, Field { attrs: [], vis: Public(VisPublic { pub_token: Pub }), ident: Some(Ident(unattred)), colon_token: Some(Colon), ty: Path(TypePath { qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident(u16), arguments: None }] } }) }, Comma] }), semi_token: None })"






// "Struct(DataStruct { struct_token: Struct, fields: Named(FieldsNamed { brace_token: Brace, named: [Field { attrs: [Attribute { pound_token: Pound, style: Outer, bracket_token: Bracket, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident(derpy), arguments: None }] }, tts: TokenStream [Punct { op: '=', spacing: Alone }, Literal { lit: 123 }] }], vis: Public(VisPublic { pub_token: Pub }), ident: Some(Ident(byte)), colon_token: Some(Colon), ty: Path(TypePath { qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident(u8), arguments: None }] } }) }, Comma, Field { attrs: [], vis: Public(VisPublic { pub_token: Pub }), ident: Some(Ident(unattred)), colon_token: Some(Colon), ty: Path(TypePath { qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident(u16), arguments: None }] } }) }, Comma] }), semi_token: None })"