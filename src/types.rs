use syn::Meta;

#[derive(Debug)]
pub struct MetaAttributes {
    pub vis: Option<String>,
    pub prefix: String,
    pub suffix: String,
}

pub struct GenParams {
    pub attribute_name: &'static str,
    pub fn_name_prefix: &'static str,
    pub fn_name_suffix: &'static str,
    pub global_attr: Option<Meta>,
}

pub enum GenMode {
    Get,
    Set,
    GetMut,
}
