use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(AsAny)]
pub fn derive_as_any(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    let output = quote! {
        #[automatically_derived]
        impl ::ray_traits::AsAny for #name {
            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }
        }
    };

    output.into()
}
