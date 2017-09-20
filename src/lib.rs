
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(EnumError)]
pub fn enum_error(input: TokenStream) -> TokenStream {
    let ast = into_ast(input);
    let froms = impl_into_enum(&ast);
    let display = impl_display(&ast);
    let error = impl_error(&ast);
    let tokens = quote!{ #froms #display #error };
    tokens.parse().unwrap()
}

#[proc_macro_derive(IntoEnum)]
pub fn into_enum(input: TokenStream) -> TokenStream {
    let ast = into_ast(input);
    impl_into_enum(&ast).parse().unwrap()
}

#[proc_macro_derive(NewType)]
pub fn newtype(input: TokenStream) -> TokenStream {
    let ast = into_ast(input);
    let from = impl_newtype_from(&ast);
    let deref = impl_newtype_deref(&ast);
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

fn impl_into_enum(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let variants = get_enum_variants(&ast);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let impls = variants.iter().map(|var| {
        let v = &var.ident;
        let cont = match var.data {
            syn::VariantData::Tuple(ref c) => c,
            _ => unreachable!(),
        };
        assert!(cont.len() == 1, "Single Tuple is required");
        let ctype = &cont[0].ty;
        quote!{
            impl #impl_generics From<#ctype> for #name #ty_generics #where_clause {
                fn from(val: #ctype) -> Self {
                    #name::#v(val)
                }
            }
        }
    });
    quote!{ #(#impls)* }
}

fn impl_newtype_from(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let field = get_basetype(&ast);
    let base = &field.ty;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote!{
        impl #impl_generics From<#base> for #name #ty_generics #where_clause {
            fn from(val: #base) -> Self {
                #name(val)
            }
        }
    }
}

fn impl_newtype_deref(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let field = get_basetype(&ast);
    let base = &field.ty;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote!{
        impl #impl_generics ::std::ops::Deref for #name #ty_generics #where_clause {
            type Target = #base;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
        impl #impl_generics ::std::ops::DerefMut for #name #ty_generics #where_clause {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
    }
}

fn impl_error(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let variants = get_enum_variants(&ast);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let snips = variants.iter().map(|var| {
        let v = &var.ident;
        quote!{ #name::#v(ref err) => err.description() }
    });
    quote!{
        impl #impl_generics ::std::error::Error for #name #ty_generics #where_clause {
            fn description(&self) -> &str {
                match *self {
                    #(#snips), *
                }
            }
        }
    }
}

fn impl_display(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let variants = get_enum_variants(&ast);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let snips = variants.iter().map(|var| {
        let v = &var.ident;
        quote!{ #name::#v(ref err) => err.fmt(f) }
    });
    quote!{
        impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    #(#snips), *
                }
            }
        }
    }
}
