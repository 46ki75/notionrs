use syn::DeriveInput;

mod setter;

#[proc_macro_derive(Setter, attributes(skip))]
pub fn struct_info_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse_macro_input!(input);
    setter::generate_setters(input)
}
