use proc_macro::TokenStream;
use syn::DeriveInput;

mod setter;

#[proc_macro_derive(Setter)]
pub fn struct_info_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse_macro_input!(input);
    setter::generate_setters(input)
}
