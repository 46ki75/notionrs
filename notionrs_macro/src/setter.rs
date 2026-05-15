use quote::quote;
use syn::{Data, DeriveInput, Expr, Fields, Lit, Meta, MetaNameValue, Type};

pub fn generate_setters(input: DeriveInput) -> proc_macro::TokenStream {
    let struct_name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named.into_iter().collect::<Vec<_>>(),
            Fields::Unnamed(unnamed) => {
                return syn::Error::new_spanned(
                    unnamed.unnamed,
                    "Setter does not support tuple structs",
                )
                .to_compile_error()
                .into();
            }
            Fields::Unit => {
                return syn::Error::new_spanned(
                    struct_name,
                    "Setter does not support unit structs",
                )
                .to_compile_error()
                .into();
            }
        },
        Data::Enum(data) => {
            return syn::Error::new_spanned(data.enum_token, "Setter can only be used with structs")
                .to_compile_error()
                .into();
        }
        Data::Union(data) => {
            return syn::Error::new_spanned(
                data.union_token,
                "Setter can only be used with structs",
            )
            .to_compile_error()
            .into();
        }
    };

    let mut errors = Vec::<syn::Error>::new();

    let field_setters = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_ty = &f.ty;
        let attribute = &f.attrs;

        let comment = generate_comment(f);

        if is_skip(attribute) {
            quote! {}
        } else if is_string_type(field_ty) {
            quote! {
                #comment
                pub fn #field_name<S>(mut self, #field_name: S) -> Self
                where
                    S: AsRef<str>,
                {
                    self.#field_name = #field_name.as_ref().to_string();
                    self
                }
            }
        } else if is_option_type(field_ty) {
            let Some(inner_ty) = option_inner_ty(field_ty) else {
                errors.push(syn::Error::new_spanned(
                    field_ty,
                    "Option type must have a generic argument",
                ));
                return quote! {};
            };

            if is_string_type(inner_ty) {
                quote! {
                    #comment
                    pub fn #field_name<S>(mut self, #field_name: S) -> Self
                    where
                        S: AsRef<str>,
                    {
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
        } else {
            quote! {
                #comment
                pub fn #field_name(mut self, #field_name: #field_ty) -> Self {
                    self.#field_name = #field_name;
                    self
                }
            }
        }
    });

    let setters: Vec<_> = field_setters.collect();

    if !errors.is_empty() {
        let combined = errors.into_iter().reduce(|mut acc, e| {
            acc.combine(e);
            acc
        });
        return combined.unwrap().to_compile_error().into();
    }

    let expanded = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #(#setters)*
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

    let field_original_comments: Vec<_> = f
        .attrs
        .iter()
        .filter_map(|attr| {
            if !attr.path().is_ident("doc") {
                return None;
            }
            if let Meta::NameValue(MetaNameValue {
                value: Expr::Lit(comment),
                ..
            }) = &attr.meta
            {
                if let Lit::Str(comment) = &comment.lit {
                    let comment = comment.value();
                    return Some(quote! { #[doc = #comment] });
                }
            }
            None
        })
        .collect();

    if field_original_comments.is_empty() {
        quote! {
            #[doc = #setter_comment]
        }
    } else {
        quote! {
            #[doc = #setter_comment]
            #[doc = ""]
            #[doc = "---"]
            #[doc = ""]
            #(#field_original_comments)*
        }
    }
}

fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn is_string_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "String";
        }
    }
    false
}

fn option_inner_ty(ty: &Type) -> Option<&Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };
    let segment = type_path.path.segments.last()?;
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    let syn::GenericArgument::Type(inner) = args.args.first()? else {
        return None;
    };
    Some(inner)
}

fn is_skip(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("setter") {
            return false;
        }
        let mut found = false;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                found = true;
            }
            Ok(())
        });
        found
    })
}
