use proc_macro::TokenStream;
use quote::ToTokens;

/// `let name = crate_name!();`
#[proc_macro]
pub fn crate_name(_: TokenStream) -> TokenStream {
    std::env::var("CARGO_PKG_NAME")
        .expect("The environment variable `CARGO_PKG_NAME` cannot be found!")
        .to_token_stream()
        .into()
}
