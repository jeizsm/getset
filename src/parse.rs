use crate::types::{GenParams, MetaAttributes};
use syn::*;

pub fn meta(meta: &Meta, params: &GenParams) -> MetaAttributes {
    let mut attributes = MetaAttributes {
        vis: None,
        prefix: params.fn_name_prefix.to_string(),
        suffix: params.fn_name_suffix.to_string(),
    };
    match meta {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(s), ..
        }) => {
            attributes.vis = Some(s.value());
            attributes
        }
        Meta::List(MetaList { nested, .. }) => {
            nested.iter().for_each(|nested_meta| {
                if let NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                    lit: Lit::Str(s),
                    ident,
                    ..
                })) = nested_meta
                {
                    match ident.to_string().as_ref() {
                        "vis" => attributes.vis = Some(s.value()),
                        "prefix" => attributes.prefix = s.value(),
                        "suffix" => attributes.suffix = s.value(),
                        _ => (),
                    }
                }
            });
            attributes
        }
        Meta::Word(_) => attributes,
        _ => attributes,
    }
}

pub fn attr_tuple(attr: &Attribute) -> Option<(Ident, Meta)> {
    let meta = attr.interpret_meta();
    meta.map(|v| (v.name(), v))
}

pub fn global_attr(attrs: &[syn::Attribute], attribute_name: &str) -> Option<Meta> {
    attrs
        .iter()
        .filter_map(|v| {
            let (attr_name, meta) = attr_tuple(v).expect("attribute");
            if attr_name == attribute_name {
                Some(meta)
            } else {
                None
            }
        }).last()
}
