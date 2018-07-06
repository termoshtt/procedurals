
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Data, DeriveInput, Field, Fields, Variant};

#[proc_macro_derive(EnumError)]
pub fn enum_error(input: TokenStream) -> TokenStream {
    let ast = into_ast(input);
    let froms = impl_into_enum(&ast);
    let display = impl_display(&ast);
    let error = impl_error(&ast);
    let tokens = quote!{ #froms #display #error };
    tokens.into()
}

#[proc_macro_derive(IntoEnum)]
pub fn into_enum(input: TokenStream) -> TokenStream {
    let ast = into_ast(input);
    let froms = impl_into_enum(&ast);
    let tokens = quote!{ #froms };
    tokens.into()
}

#[proc_macro_derive(NewType)]
pub fn newtype(input: TokenStream) -> TokenStream {
    let ast = into_ast(input);
    let from = impl_newtype_from(&ast);
    let deref = impl_newtype_deref(&ast);
    let tokens = quote!{ #from #deref };
    tokens.into()
}

fn into_ast(input: TokenStream) -> DeriveInput {
    syn::parse(input).unwrap()
}

fn get_enum_variants(ast: &DeriveInput) -> Vec<&Variant> {
    match ast.data {
        Data::Enum(ref inner) => inner.variants.iter().collect(),
        Data::Struct(_) => unreachable!("Structs are not supported"),
        Data::Union(_) => unreachable!("Unions are not supported"),
    }
}

fn get_basetype(ast: &DeriveInput) -> &Field {
    match ast.data {
        Data::Enum(_) => unreachable!("Enums are not supported"),
        Data::Union(_) => unreachable!("Unions are not supported"),
        Data::Struct(ref ds) => {
            match ds.fields {
                Fields::Named(_) => unreachable!("Must be tuple struct"),
                Fields::Unnamed(ref t) => {
                    if t.unnamed.len() > 1 {
                        unreachable!("Must be one type");
                    }
                    &t.unnamed[0]
                }
                Fields::Unit => unreachable!("Must be tuple struct"),
            }
        }
    }
}

fn impl_into_enum(ast: &DeriveInput) -> Box<ToTokens> {
    let name = &ast.ident;
    let variants = get_enum_variants(&ast);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let impls = variants.iter().map(|var| {
        let v = &var.ident;
        let cont = match var.fields {
            Fields::Unnamed(ref c) => &c.unnamed,
            _ => unreachable!(),
        };
        assert!(cont.len() == 1, "Single tuple is required");
        let ctype = &cont[0].ty;
        quote!{
            impl #impl_generics From<#ctype> for #name #ty_generics #where_clause {
                fn from(val: #ctype) -> Self {
                    #name::#v(val)
                }
            }
        }
    });
    Box::new(quote!{ #(#impls)* })
}

fn impl_newtype_from(ast: &DeriveInput) -> Box<ToTokens> {
    let name = &ast.ident;
    let field = get_basetype(&ast);
    let base = &field.ty;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    Box::new(
        quote!{
            impl #impl_generics From<#base> for #name #ty_generics #where_clause {
                fn from(val: #base) -> Self {
                    #name(val)
                }
            }

            impl #impl_generics Into<#base> for #name #ty_generics #where_clause {
                fn into(self) -> #base {
                    self.0
                }
            }
        }
    )
}

fn impl_newtype_deref(ast: &DeriveInput) -> Box<ToTokens> {
    let name = &ast.ident;
    let field = get_basetype(&ast);
    let base = &field.ty;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    Box::new(
        quote!{
            impl #impl_generics ::std::ops::Deref for #name #ty_generics #where_clause {
                type Target = #base;
                fn deref(&self) -> &Self::Target { &self.0 }
            }
            impl #impl_generics ::std::ops::DerefMut for #name #ty_generics #where_clause {
                fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
            }
        }
    )
}

fn impl_error(ast: &DeriveInput) -> Box<ToTokens> {
    let name = &ast.ident;
    let variants = get_enum_variants(&ast);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let snips = variants.iter().map(|var| {
        let v = &var.ident;
        quote!{ #name::#v(ref err) => err.description() }
    });
    Box::new(
        quote!{
            impl #impl_generics ::std::error::Error for #name #ty_generics #where_clause {
                fn description(&self) -> &str {
                    match *self {
                        #(#snips), *
                    }
                }
            }
        }
    )
}

fn impl_display(ast: &DeriveInput) -> Box<ToTokens> {
    let name = &ast.ident;
    let variants = get_enum_variants(&ast);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let snips = variants.iter().map(|var| {
        let v = &var.ident;
        quote!{ #name::#v(ref err) => err.fmt(f) }
    });
    Box::new(
        quote!{
            impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        #(#snips), *
                    }
                }
            }
        }
    )
}
