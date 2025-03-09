use core::panic;

use quote::quote;
use syn::{Data, DeriveInput, Expr, Fields, Lit, Meta, MetaNameValue};

pub fn generate_setters(input: DeriveInput) -> proc_macro::TokenStream {
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

        let comment = generate_comment(f);

        if field_ty == &syn::parse_str("String").unwrap()
        // String
        {
            quote! {
                #comment
                pub fn #field_name<T>(mut self, #field_name: T) -> Self
                where
                    T: AsRef<str>,
                {
                    self.#field_name = #field_name.as_ref().to_string();
                    self
                }
            }
        } else if is_option_type(field_ty)
        // Option<T>
        {
            let inner_ty = if let syn::Type::Path(type_path) = field_ty {
                if let Some(first_segment) = type_path.path.segments.first() {
                    if let syn::PathArguments::AngleBracketed(args) = &first_segment.arguments {
                        if let syn::GenericArgument::Type(ty) = args.args.first().unwrap() {
                            ty
                        } else {
                            panic!("Option type must have a generic argument");
                        }
                    } else {
                        panic!("Option type must have a generic argument");
                    }
                } else {
                    panic!("Option type must have a generic argument");
                }
            } else {
                panic!("Option type must have a generic argument");
            };

            if inner_ty == &syn::parse_str("String").unwrap()
            // Option<String>
            {
                quote! {
                    #comment
                    pub fn #field_name<T>(mut self, #field_name: T) -> Self
                    where
                        T: AsRef<str>,{
                        self.#field_name = Some(#field_name.as_ref().to_string());
                        self
                    }
                }
            } else {
                quote! {
                    #comment
                    pub fn #field_name(mut self, #field_name: #inner_ty) -> Self {
                        self.#field_name = Some(#field_name);
                        self
                    }
                }
            }
        } else
        // others
        {
            quote! {
                #comment
                pub fn #field_name(mut self, #field_name: #field_ty) -> Self {
                    self.#field_name = #field_name;
                    self
                }
            }
        }
    });

    let expanded = quote! {
        impl #struct_name {
            #(#field_setters)*
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn generate_comment(f: &syn::Field) -> proc_macro2::TokenStream {
    let field_name = &f.ident;
    let setter_comment = format!(
        "Set the value of the `{}` field.",
        field_name.as_ref().unwrap()
    );

    let field_original_comment = f.attrs.iter().filter_map(|attr| {
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

            return None;
        }
        None
    });

    let comment = quote! {
        #[doc = #setter_comment]
        #[doc = ""]
        #[doc = "---"]
        #[doc = ""]
        #(#field_original_comment)*
    };

    comment
}

fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(first_segment) = type_path.path.segments.first() {
            return first_segment.ident == "Option";
        }
    }
    false
}
