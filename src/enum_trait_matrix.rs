use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Field, Fields, FieldsNamed, FieldsUnnamed, FnArg, ItemEnum, ItemMod, ItemTrait, PatType, Receiver, Signature, TraitItem, TraitItemFn, Variant
};

use crate::parse::ParsedModule;


pub fn enum_trait_matrix(module: ItemMod) -> TokenStream {
    let ParsedModule { vis, name, r#enum, r#trait } = ParsedModule::parse(module);
    
    let structs = quote_structs(r#enum.clone());
    let froms = quote_froms(r#enum.clone());
    let r#impl = quote_impl(r#enum.clone(), r#trait.clone());
    let r#enum = quote_enum(r#enum);
    let r#trait = quote_trait(r#trait);
    
    quote!{
        #vis mod #name {
            #structs
            #froms
            #r#enum
            #r#impl
            #r#trait
        }
    }
}


pub fn quote_structs(r#enum: ItemEnum) -> TokenStream {
    let attrs = r#enum.attrs;
    
    let structs = r#enum.variants.into_iter().map(|v| {
        let Variant { ident, fields, .. } = v;
        match fields {
            Fields::Named(FieldsNamed { named, .. }) => {
                let fields = named.into_iter().map(|Field { ident, ty, .. }| {
                    quote!{ pub #ident: #ty }
                });
                
                quote!{
                    #(#attrs)*
                    pub struct #ident{#(#fields,)*}
                }
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let fields = unnamed.into_iter().map(|Field { ty, .. }| {
                    quote!{ pub #ty }
                });
                
                quote!{
                    #(#attrs)*
                    pub struct #ident(#(#fields,)*);
                }
            }
            Fields::Unit => quote!{
                #(#attrs)*
                pub struct #ident;
            },
        }
    });
    
    quote!{
        #(#structs)*
    }
}

pub fn quote_froms(r#enum: ItemEnum) -> TokenStream {
    let enum_name = r#enum.ident;
    let froms = r#enum.variants.into_iter().map(|v| {
        let Variant { ident: name, fields, .. } = v;
        match fields {
//            Fields::Named(FieldsNamed { named, .. }) => {
//                let fields = named.into_iter().map(|Field { ident, .. }| {
//                    ident
//                });
//                
//                let fields2 = fields.clone();
//                
//                quote!{
//                    impl std::convert::From<#name> for #enum_name {
//                        fn from(#name{ #(#fields,)* }: #name) -> Self {
//                            Self::#name { #(#fields2,)* }
//                        }
//                    }
//                }
//            }
//            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
//                let fields = (0..unnamed.len()).map(syn::Index::from);
//                
//                quote!{
//                    impl std::convert::From<#name> for #enum_name {
//                        fn from(value: #name) -> Self {
//                            Self::#name( #(value.#fields,)* )
//                        }
//                    }
//                }
//            }
            Fields::Unit => quote!{
                impl std::convert::From<#name> for #enum_name {
                    fn from(_: #name) -> Self {
                        Self::#name
                    }
                }
            },
            _ => quote!{
                impl std::convert::From<#name> for #enum_name {
                    fn from(value: #name) -> Self {
                        Self::#name(value)
                    }
                }
            },
        }
    });
    
    quote!{ #(#froms)* }
}

pub fn quote_enum(r#enum: ItemEnum) -> TokenStream {
    let attrs = r#enum.attrs;
    let name = r#enum.ident;
    
    let variants = r#enum.variants.into_iter().map(|v| {
        let Variant { ident, fields, .. } = v;
        match fields {
            Fields::Unit => quote!{
                #ident,
            },
            _ => quote!{
                #ident(#ident),
            },
        }
    });
    
    quote!{
        #(#attrs)*
        pub enum #name {
            #(
                #variants
            )*
        }
    }
}

pub fn quote_impl(r#enum: ItemEnum, r#trait: ItemTrait) -> TokenStream {
    let enum_name = r#enum.ident;
    let trait_name = r#trait.ident;
    
    let structs: Vec<_> = r#enum.variants.into_iter().map(|Variant { ident: name, fields, .. }| -> (TokenStream, Box<dyn Fn(&TokenStream) -> TokenStream>) {
        match fields {
            Fields::Unit => (quote!{ #name }, Box::new(move |sigil| quote!{ #sigil #name })),
            _ => (quote!{ #name(__self) }, Box::new(|_| quote!{ __self }) )
        }
    }).collect();
    
    let methods = r#trait.items.into_iter().filter_map(|i| match i { TraitItem::Fn(r#fn) => Some(r#fn), _ => None }).map(|i| {
        let TraitItemFn { sig, .. } = i;
        let Signature { ident, inputs, .. } = sig.clone();
        
        let sigil = if let FnArg::Receiver(Receiver { reference, mutability, .. }) = &inputs[0] {
            match (reference.is_some(), mutability.is_some()) {
                (true, true) => quote!{ &mut },
                (true, false) => quote!{ & },
                _ => quote!{},
            }
        } else {
            quote!{}
        };
        
        let args: Vec<_> = inputs.into_iter().skip(1).filter_map(|i| {
            match i {
                FnArg::Receiver(_) => {
                    None
                }
                FnArg::Typed(PatType { pat, .. }) => Some(*pat),
            }
        }).collect();
        
        let branches = structs.iter().map(|s| {
            let (a, b) = s;
            
            let b = b(&sigil);
            
            quote!{
                Self::#a => {
                    #trait_name::#ident(#b, #(#args,)* )
                }
            }
        });
        
        quote!{
            #sig {
                match self {
                    #( #branches )*
                }
            }
        }
    });
    
    quote!{
        impl #trait_name for #enum_name {
            #( #methods )*
        }
    }
}

pub fn quote_trait(r#trait: ItemTrait) -> TokenStream {
    let name = r#trait.ident;
    let items = r#trait.items;
    
    quote!{
        pub trait #name {
            #(#items)*
        }
    }
}
