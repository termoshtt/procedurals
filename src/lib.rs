
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(EnumError)]
pub fn enum_error(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = impl_enum_error(&ast);
    gen.parse().unwrap()
}

#[proc_macro_derive(IntoEnum)]
pub fn into_enum(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = impl_into_enum(&ast);
    gen.parse().unwrap()
}

fn impl_from_traits(name: &syn::Ident, variants: &Vec<syn::Variant>) -> quote::Tokens {
    let impls = variants.iter().map(|var| {
        let v = &var.ident;
        let cont = match var.data {
            syn::VariantData::Tuple(ref c) => c,
            _ => unreachable!(),
        };
        assert!(cont.len() == 1, "Single Tuple is required");
        let ctype = &cont[0].ty;
        quote!{
                impl From<#ctype> for #name {
                    fn from(val: #ctype) -> Self {
                        #name::#v(val)
                    }
                }
            }
    });
    quote!{ #(#impls)* }
}

fn impl_error(name: &syn::Ident, variants: &Vec<syn::Variant>) -> quote::Tokens {
    let snips = variants.iter().map(|var| {
        let v = &var.ident;
        quote!{ #name::#v(ref err) => err.description() }
    });
    quote!{
        impl ::std::error::Error for #name {
            fn description(&self) -> &str {
                match *self {
                    #(#snips), *
                }
            }
        }
    }
}

fn impl_display(name: &syn::Ident, variants: &Vec<syn::Variant>) -> quote::Tokens {
    let snips = variants.iter().map(|var| {
        let v = &var.ident;
        quote!{ #name::#v(ref err) => err.fmt(f) }
    });
    quote!{
        impl ::std::fmt::Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    #(#snips), *
                }
            }
        }
    }
}

fn impl_enum_error(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let ref variants = match ast.body {
        syn::Body::Enum(ref variants) => variants,
        syn::Body::Struct(_) => unreachable!(),
    };
    let mut token = quote::Tokens::new();
    token.append_all(
        &[
            impl_from_traits(&name, &variants),
            impl_display(&name, &variants),
            impl_error(&name, &variants),
        ],
    );
    token
}

fn impl_into_enum(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let ref variants = match ast.body {
        syn::Body::Enum(ref variants) => variants,
        syn::Body::Struct(_) => unreachable!(),
    };
    let mut token = quote::Tokens::new();
    token.append_all(&[impl_from_traits(&name, &variants)]);
    token
}
