
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

fn into_ast(input: TokenStream) -> syn::DeriveInput {
    let s = input.to_string();
    syn::parse_macro_input(&s).unwrap()
}

#[proc_macro_derive(EnumError)]
pub fn enum_error(input: TokenStream) -> TokenStream {
    let ast = into_ast(input);
    let name = &ast.ident;
    let variants = get_enum_variants(&ast);
    let froms = impl_froms(&name, &variants);
    let display = impl_display(&name, &variants);
    let error = impl_error(&name, &variants);
    let tokens = quote!{ #froms #display #error };
    tokens.parse().unwrap()
}

#[proc_macro_derive(IntoEnum)]
pub fn into_enum(input: TokenStream) -> TokenStream {
    let ast = into_ast(input);
    let name = &ast.ident;
    let variants = get_enum_variants(&ast);
    impl_froms(&name, &variants).parse().unwrap()
}

fn get_enum_variants(ast: &syn::MacroInput) -> &Vec<syn::Variant> {
    match ast.body {
        syn::Body::Enum(ref variants) => variants,
        syn::Body::Struct(_) => unreachable!("Struct is not supported"),
    }
}

fn impl_from(from: &syn::Ty, to: &syn::Ident, val: &syn::Ident) -> quote::Tokens {
    quote!{
        impl From<#from> for #to {
            fn from(val: #from) -> Self {
                #to::#val(val)
            }
        }
    }
}

fn impl_froms(name: &syn::Ident, variants: &Vec<syn::Variant>) -> quote::Tokens {
    let impls = variants.iter().map(|var| {
        let v = &var.ident;
        let cont = match var.data {
            syn::VariantData::Tuple(ref c) => c,
            _ => unreachable!(),
        };
        assert!(cont.len() == 1, "Single Tuple is required");
        let ctype = &cont[0].ty;
        impl_from(ctype, name, v)
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
