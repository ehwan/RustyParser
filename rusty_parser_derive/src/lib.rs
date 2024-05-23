use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// must implement: seq, callback, or, repeat

// ResultVoid
#[proc_macro_derive(ResultVoid)]
pub fn derive_result_void(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ResultVoid for #name #ty_generics #where_clause {
        }
    };

    TokenStream::from(expanded)
}

// ResultValue
#[proc_macro_derive(ResultValue)]
pub fn derive_result_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ResultValue for #name #ty_generics #where_clause {
        }
    };

    TokenStream::from(expanded)
}

// ResultTuple
#[proc_macro_derive(ResultTuple)]
pub fn derive_result_tuple(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ResultTuple for #name #ty_generics #where_clause {
        }
    };

    TokenStream::from(expanded)
}
