
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

fn impl_enum_error(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let ref variants = match ast.body {
        syn::Body::Enum(ref variants) => variants,
        syn::Body::Struct(_) => unreachable!(),
    };
    variants.iter()
        .map(|var| {
            let cont = match var.data {
                syn::VariantData::Tuple(ref c) => c,
                _ => unreachable!(),
            };
            let v = &var.ident;
            let ctype = &cont[0].ty;
            quote!{
                impl From<#ctype> for #name {
                    fn from(val: #ctype) -> Self {
                        #name::#v(val)
                    }
                }
            }
        })
        .fold(quote::Tokens::new(), |mut cum, a| {
            cum.append(a);
            cum
        })
}
