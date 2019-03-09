use syn::parse::{Parse, ParseStream};
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::{LitInt, NestedMeta, Meta, parse_macro_input, DeriveInput, Data, Fields, Generics, GenericParam, parse_quote, Index, Lit, LitStr, Variant, ItemStruct, FieldsNamed, Ident, ItemEnum, braced, Token};
use syn::spanned::Spanned;
use syn::punctuated::Punctuated;
use proc_macro2::{TokenStream, Span};


pub mod util;



// mod kw {
//     syn::custom_keyword!(<=>);
// }

#[derive(Debug)]
pub struct SpaceshipOp {
    pub lt_token: Token![<],
    pub eq_token: Token![=],
    pub gt_token: Token![>],
}

impl Parse for SpaceshipOp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(SpaceshipOp {
            lt_token: input.parse()?,
            eq_token: input.parse()?,
            gt_token: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct LiteralClosedRange {
    pub lo: syn::Lit,
    pub sep: Token![..=],
    pub hi: syn::Lit,
}

impl Parse for LiteralClosedRange {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(LiteralClosedRange {
            lo: input.parse()?,
            sep: input.parse()?,
            hi: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub enum LiteralOrRange {
    Literal(syn::Lit),
    Range(LiteralClosedRange),
}

impl Parse for LiteralOrRange {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // let lookahead = input.lookahead1();
        if input.peek(syn::Lit) {
            if input.peek2(Token![..]){
                input.parse().map(LiteralOrRange::Range)
            } else {
                input.parse().map(LiteralOrRange::Literal)
            }
        } else {
            Err(input.error("Must see literal first"))
        }
    }
}
pub type LiteralPatterns = Punctuated<LiteralOrRange, Token![|]>;

#[derive(Debug)]
pub enum LiteralOrPatterns {
    Literal(syn::Lit),
    MultiValued(LiteralPatterns),
}

impl Parse for LiteralOrPatterns {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let patterns = input.call(LiteralPatterns::parse_separated_nonempty)?;
        // Safe to get 0 index because the parsing method used aboved accepts one or more only
        let first = &patterns[0];
        Ok(match (patterns.len(), first) {
            (0, _) => unreachable!(),
            (1, LiteralOrRange::Literal(lit)) => LiteralOrPatterns::Literal((*lit).clone()), 
            _ => LiteralOrPatterns::MultiValued(patterns)
        })
    }
}

#[derive(Debug)]
pub struct BidirectionalVariant {
    pub attrs: Vec<syn::Attribute>,
    pub ident: Ident,
    pub spaceship_op: SpaceshipOp,
    pub lit_or_patterns: LiteralOrPatterns
}


impl Parse for BidirectionalVariant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(BidirectionalVariant {
            attrs: input.call(syn::Attribute::parse_outer)?,
            ident: input.parse()?,
            spaceship_op: input.parse()?,
            lit_or_patterns: input.parse()?,
        })
    }
}



pub struct BidirectionalVariantToTokensAdapter<'a, 'b> {
    variant: &'a BidirectionalVariant,
    ty: &'b syn::Type,
}


impl<'a, 'b> ToTokens for BidirectionalVariantToTokensAdapter<'a, 'b> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.variant.attrs);
        self.variant.ident.to_tokens(tokens);
        if let LiteralOrPatterns::MultiValued(_) = self.variant.lit_or_patterns {
            syn::token::Paren { span: Span::call_site() }
                .surround(tokens, |tokens| {
                    self.ty.to_tokens(tokens)
                })
        }
    }
}



#[derive(Debug)]
pub struct BidirectionalEnum {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub enum_token: Token![enum],
    pub ident: Ident,
    pub colon_token: Token![:],
    pub mapped_type: syn::Type,
    pub brace_token: syn::token::Brace,
    pub variants: Punctuated<BidirectionalVariant, Token![,]>
}


impl Parse for BidirectionalEnum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(BidirectionalEnum {
            attrs: input.call(syn::Attribute::parse_outer)?,
            vis: input.parse()?,
            enum_token: input.parse()?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            mapped_type: input.parse()?,
            brace_token: braced!(content in input),
            variants: content.parse_terminated(BidirectionalVariant::parse)?
        })
    }
}

impl BidirectionalEnum {
    fn from_enum_impl_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let from_type = &self.mapped_type;
        let from_body_enum = quote! {

        };
        tokens.extend(quote! {
            impl From<#from_type> for #ident {
                fn from(from_data: #from_type) -> Self {

                }
            }
        })
        // syn::token::Impl { span: Span::call_site() }
        //     .to_tokens(tokens);
        
        
    }
}

impl ToTokens for BidirectionalEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.attrs);
        self.vis.to_tokens(tokens);
        self.enum_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            self.variants.pairs()
                .map(|pair| match pair {
                    syn::punctuated::Pair::Punctuated(variant, _) | syn::punctuated::Pair::End(variant)  => syn::punctuated::Pair::Punctuated(BidirectionalVariantToTokensAdapter {
                        variant: &variant,
                        ty: &self.mapped_type
                    }, syn::token::Comma {spans: [Span::call_site()]}),
                })
                .collect::<Punctuated<BidirectionalVariantToTokensAdapter, Token![,]>>()
             .to_tokens(tokens);  
        })

    }
}



// pub enum AddFieldsMode {
//     Append,
//     Prepend,
//     AtIndex(usize),
// }

pub enum AddFieldsAttr<'a> {
    Append,
    Prepend,
    AtIndex(&'a LitInt)
}



impl<'a> Default for AddFieldsAttr<'a> {
    fn default() -> Self { AddFieldsAttr::Append }
}








// #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]




// Meta(
//         Word(
//             Ident {
//                 ident: "append",
//                 span: #0 bytes(134..140)
//             }
//         )
//     )


// Meta(
//         List(
//             MetaList {
//                 ident: Ident {
//                     ident: "AtIndex",
//                     span: #0 bytes(134..141)
//                 },
//                 paren_token: Paren,
//                 nested: [
//                     Literal(
//                         Int(
//                             LitInt {
//                                 token: Literal { lit: Integer(3), suffix: None, span: Span { lo: BytePos(142), hi: BytePos(143), ctxt: #0 } }
//                             }
//                         )
//                     )
//                 ]
//             }
//         )
//     )
impl<'a> AddFieldsAttr<'a> {
    pub fn new(metas: &'a [NestedMeta]) -> syn::Result<Self> {
        match metas {
            [] => Ok(Default::default()),
            [NestedMeta::Meta(Meta::Word(ident))] => {
                let ident_str = ident.to_string();
                match ident_str.as_ref() {
                    "Append" => Ok(AddFieldsAttr::Append),
                    "Prepend" => Ok(AddFieldsAttr::Prepend),
                    _ => Err(syn::Error::new(ident.span(), format!("Invalid append mode `{}`", ident_str))),
                }
            },
            [NestedMeta::Meta(Meta::List(meta_list))] => {
                let ident_str = meta_list.ident.to_string();
                if ident_str == "AtIndex" {
                    let neted_meta_list: Vec<&NestedMeta> = (&meta_list.nested).into_iter().collect();
                    let neted_meta_list : &[&NestedMeta] = neted_meta_list.as_ref();
                    match neted_meta_list {
                        [NestedMeta::Literal(Lit::Int(lit_int))] => {
                            let lit_value = lit_int.value();
                            if (lit_value as usize) as u64 == lit_value {
                                Ok(AddFieldsAttr::AtIndex(lit_int))
                            } else {
                                Err(syn::Error::new(lit_int.span(), "{} does not fit into usize"))
                            }
                        }
                        _ => Err(syn::Error::new(meta_list.span(), "Invalid argument(s) passed to AtIndex attribute meta"))
                    }
                } else {
                    Err(syn::Error::new(meta_list.ident.span(), format!("Invalid append mode `{}`", ident_str)))
                }
            }
            _ => Err(syn::Error::new(metas[0].span(), format!("derpy {:?}", metas)))
        }
    }
}



// impl Parse for AddFieldsMode {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         if input.is_empty() {
//             Ok(Default::default())
//         } else {
//             input.parse::<>()
//         }
//     }
// }



pub fn backend_derive_derp(input: &DeriveInput) -> TokenStream {
    let derp = 1;
    TokenStream::new()
}


fn insert_fields_at(attr: AddFieldsAttr, struct_fields: &FieldsNamed, new_fields: &TokenStream) -> syn::Result<FieldsNamed> {
    let named = &struct_fields.named;

    let num_fields = named.len();
    let index = match attr {
        AddFieldsAttr::Append => num_fields,
        AddFieldsAttr::Prepend => 0,
        AddFieldsAttr::AtIndex(lit_int) => {
            let value = lit_int.value() as usize;
            if value > num_fields {
                return Err(syn::Error::new(lit_int.span(), format!("Cannot insert at {}, currently only {} fields", value, named.len())))
            }
            value
        }
    };

    
    let before = named
        .into_iter()
        .take(index);

    let after = named
        .into_iter()
        .skip(index);


    let modded_fields =  quote! ({
        #(#before,)*
        #new_fields
        #(#after,)*
    });
    syn::parse2(modded_fields)
    // return Err(syn::Error::new(struct_fields.named[0].span(), format!("Derpy \n {} \n\n\n\n\n {:#?}", modded_fields, modded_fields)));
    // let modded_struct = syn::parse2::<FieldsNamed>(modded_fields)?;
    // Err(syn::Error::new(fields.named[0].span(), format!("Derpy {:#?}", modded_struct)))
    
}


// Meta(
//         Word(
//             Ident {
//                 ident: "append",
//                 span: #0 bytes(134..140)
//             }
//         )
//     )



// Meta(
//         List(
//             MetaList {
//                 ident: Ident {
//                     ident: "AtIndex",
//                     span: #0 bytes(134..141)
//                 },
//                 paren_token: Paren,
//                 nested: [
//                     Literal(
//                         Int(
//                             LitInt {
//                                 token: Literal { lit: Integer(3), suffix: None, span: Span { lo: BytePos(142), hi: BytePos(143), ctxt: #0 } }
//                             }
//                         )
//                     )
//                 ]
//             }
//         )
//     )




pub fn backend_add_fields(attr: &[NestedMeta], struct_input: &ItemStruct, new_fields: &TokenStream) -> syn::Result<TokenStream> {

    // return Err(syn::Error::new(struct_input.span(), format!("Attribute Input: {:#?}", attr)));
    let res = match  struct_input.fields {
        Fields::Named(ref struct_fields) => {
            let attr  = AddFieldsAttr::new(attr)?;
            insert_fields_at(attr, &struct_fields, new_fields)
        },
        _ => Err(syn::Error::new(struct_input.span(), "Bad add_fields content"))
    };

    let modded_fields = res?;
    let modded_struct = ItemStruct {
        fields: Fields::Named(modded_fields),
        ..struct_input.clone()
    };
    Ok(modded_struct.into_token_stream())
    
}

pub fn backend_enum_type(attr: &[NestedMeta], enum_item: &ItemEnum) -> syn::Result<()> {
    // match attr {
    //     [] =>
    // }

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_struct() {

        let raw = "
        #[derive(Debug, Derp)]
        pub struct SomeDerpStructDeriver {
            pub byte: u8,
            pub unattred: u16,
        }
        ";
        // let shitballz = 
        let input = syn::parse_str::<DeriveInput>(raw);

        let huh = 1;
        // backend_derive_derp(&input);
    }
    
}