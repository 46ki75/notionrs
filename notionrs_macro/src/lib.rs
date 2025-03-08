use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Fields, Lit, Meta, MetaNameValue, parse_macro_input};

#[proc_macro_derive(Setter)]
pub fn struct_info_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named.into_iter().collect::<Vec<_>>(),
            _ => vec![],
        },
        _ => panic!("Setter can only be used with structs"),
    };

    let field_setters = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_ty = &f.ty;
        let doc_comment = format!(
            "Set the value of the `{}` field.",
            field_name.as_ref().unwrap()
        );
        let field_comment = f.attrs.iter().filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let Meta::NameValue(MetaNameValue {
                    path: _,
                    eq_token: _,
                    value: Expr::Lit(comment),
                }) = &attr.meta
                {
                    if let Lit::Str(comment) = &comment.lit {
                        let comment = comment.value();
                        return Some(quote! {
                            #[doc = #comment]
                        });
                    }
                }

                todo!()
            }
            None
        });

        quote! {
            #[doc = #doc_comment]
            #[doc = ""]
            #[doc = "---"]
            #[doc = ""]
            #(#field_comment)*
            pub fn #field_name(mut self, #field_name: #field_ty) -> Self {
                self.#field_name = #field_name;
                self
            }
        }
    });

    let expanded = quote! {
        impl #struct_name {
            #(#field_setters)*
        }
    };

    TokenStream::from(expanded)
}
