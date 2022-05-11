use proc_macro2::TokenStream;
use quote::quote;

#[no_mangle]
pub extern "C" fn co(input: TokenStream) -> TokenStream {
    let input: syn::Expr = syn::parse2(input).unwrap();
    quote!(#input)
}
