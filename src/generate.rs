use parse;
use proc_macro2::{Span, TokenStream};
use syn::{Field, Ident, Visibility};
use types::{GenMode, GenParams};

pub fn implement(field: &Field, mode: &GenMode, params: &GenParams) -> TokenStream {
    let field_name = field
        .clone()
        .ident
        .expect("Expected the field to have a name");
    let ty = field.ty.clone();

    let mut doc = Vec::new();
    let attr = field
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
        }).last()
        .or_else(|| params.global_attr.clone())
        .expect("attribute");

    let attributes = parse::meta(&attr, params);
    let visibility: Option<Visibility> = attributes
        .vis
        .map(|vis| syn::parse_str(vis.as_ref()).expect("visibility"));
    let fn_name = Ident::new(
        &format!("{}{}{}", attributes.prefix, field_name, attributes.suffix),
        Span::call_site(),
    );
    match mode {
        GenMode::Get => {
            quote! {
                #(#doc)*
                #[inline(always)]
                #visibility fn #fn_name(&self) -> &#ty {
                    &self.#field_name
                }
            }
        }
        GenMode::Set => {
            quote! {
                #(#doc)*
                #[inline(always)]
                #visibility fn #fn_name(&mut self, val: #ty) -> &mut Self {
                    self.#field_name = val;
                    self
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
}
