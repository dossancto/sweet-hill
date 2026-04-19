use bevy::prelude::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error, Path, parse_macro_input};

#[proc_macro_derive(CollectTrigger, attributes(collect_event))]
pub fn collect_action_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    // 1. Find the #[collect_event(Type)] attribute
    let Some(attr) = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("collect_event"))
    else {
        return Error::new_spanned(&input, "Missing #[collect_event(Type)] attribute")
            .to_compile_error()
            .into();
    };

    // 2. Parse the inner type (e.g., "MyEvent")
    let output_ty = match attr.parse_args::<syn::Type>() {
        Ok(ty) => ty,
        Err(e) => return e.to_compile_error().into(),
    };

    let struct_name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::inventory_core::CollectItemAction for #struct_name #type_generics
        where
            #output_ty: Default,
            #where_clause
        {
            type Output = #output_ty;

            fn get_collect_event(&self) -> Self::Output {
                <#output_ty as Default>::default()
            }
        }
    };

    TokenStream::from(expanded)
}
