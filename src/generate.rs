use crate::parse;
use crate::types::{GenMode, GenParams};
use proc_macro2::{Span, TokenStream};
use syn::{Field, Ident, Visibility};

pub fn implement(field: &Field, mode: &GenMode, params: &GenParams) -> TokenStream {
    let field_name = field
        .clone()
        .ident
        .expect("Expected the field to have a name");
    let ty = field.ty.clone();

    let mut doc = Vec::new();
    let mut attrs: Vec<_> = field
        .attrs
        .iter()
        .filter_map(|v| {
            let (attr_name, meta) = parse::attr_tuple(v).expect("attribute");
            if attr_name == "doc" {
                doc.push(v);
                None
            } else if attr_name == params.attribute_name {
                Some(meta)
            } else {
                None
            }
        }).collect();
    if attrs.is_empty() {
        attrs = params.global_attr.clone();
    }
    let doc = &doc;

    let token_stream: Vec<_> = attrs
        .iter()
        .map(|attr| {
            let attributes = parse::meta(&attr, params);
            let visibility: Option<Visibility> = attributes
                .vis
                .map(|vis| syn::parse_str(vis.as_ref()).expect("visibility"));
            let fn_name = Ident::new(
                &format!(
                    "{}{}{}",
                    attributes.prefix.unwrap_or_default(),
                    field_name,
                    attributes.suffix.unwrap_or_default()
                ),
                Span::call_site(),
            );
            match mode {
                GenMode::Get => {
                    let (fn_type, fn_body) = if attributes.mutable {
                        (
                            quote! { (&mut self) -> &mut #ty },
                            quote! { &mut self.#field_name },
                        )
                    } else if attributes.copy {
                        (quote! { (&self) -> #ty }, quote! { self.#field_name })
                    } else {
                        (quote! { (&self) -> &#ty }, quote! { &self.#field_name })
                    };
                    quote! {
                        #(#doc)*
                        #[inline(always)]
                        #visibility fn #fn_name#fn_type {
                            #fn_body
                        }
                    }
                }
                GenMode::Set => {
                    let (fn_type, fn_body) = if attributes.consume {
                        (
                            quote! { (mut self, val: #ty) -> Self },
                            quote! {
                                self.#field_name = val;
                                self
                            },
                        )
                    } else {
                        (
                            quote! { (&mut self, val: #ty) -> &mut Self },
                            quote! {
                                self.#field_name = val;
                                self
                            },
                        )
                    };
                    quote! {
                        #(#doc)*
                        #[inline(always)]
                        #visibility fn #fn_name#fn_type {
                            #fn_body
                        }
                    }
                }
                GenMode::GetMut => {
                    quote! {
                        #(#doc)*
                        #[inline(always)]
                        #visibility fn #fn_name(&mut self) -> &mut #ty {
                            &mut self.#field_name
                        }
                    }
                }
            }
        }).collect();
    quote! {
        #(#token_stream)*
    }
}
