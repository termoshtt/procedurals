
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(EnumError)]
pub fn enum_error(input: TokenStream) -> TokenStream {
    let ast = into_ast(input);
    let name = &ast.ident;
    let variants = get_enum_variants(&ast);
    let froms = impl_into_enum(&name, &variants);
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
    impl_into_enum(&name, &variants).parse().unwrap()
}

#[proc_macro_derive(NewType)]
pub fn newtype(input: TokenStream) -> TokenStream {
    let ast = into_ast(input);
    let name = &ast.ident;
    let field = get_basetype(&ast);
    let from = impl_newtype_from(&name, &field);
    let deref = impl_newtype_deref(&name, &field);
    let tokens = quote!{ #from #deref };
    tokens.parse().unwrap()
}

fn into_ast(input: TokenStream) -> syn::DeriveInput {
    let s = input.to_string();
    syn::parse_macro_input(&s).unwrap()
}

fn get_enum_variants(ast: &syn::MacroInput) -> &Vec<syn::Variant> {
    match ast.body {
        syn::Body::Enum(ref variants) => variants,
        syn::Body::Struct(_) => unreachable!("Struct is not supported"),
    }
}

fn get_basetype(ast: &syn::MacroInput) -> &syn::Field {
    match ast.body {
        syn::Body::Enum(_) => unreachable!("Enum is not supported"),
        syn::Body::Struct(ref vd) => {
            match *vd {
                syn::VariantData::Struct(_) => unreachable!("Must be tuple"),
                syn::VariantData::Tuple(ref t) => {
                    if t.len() > 1 {
                        unreachable!("Must be one type");
                    }
                    &t[0]
                }
                syn::VariantData::Unit => unreachable!("Must be tuple"),
            }
        }
    }
}

fn impl_into_enum(name: &syn::Ident, variants: &Vec<syn::Variant>) -> quote::Tokens {
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

fn impl_newtype_from(name: &syn::Ident, field: &syn::Field) -> quote::Tokens {
    let base = &field.ty;
    quote!{
        impl From<#base> for #name {
            fn from(val: #base) -> Self {
                #name(val)
            }
        }
    }
}

fn impl_newtype_deref(name: &syn::Ident, field: &syn::Field) -> quote::Tokens {
    let base = &field.ty;
    quote!{
        impl ::std::ops::Deref for #name {
            type Target = #base;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
        impl ::std::ops::DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
    }

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
