/*!
Getset, we're ready to go!

A procedural macro for generating the most basic getters and setters on fields.

Getters are generated as `fn field(&self) -> &type`, while setters are generated as `fn field(&mut self, val: type)`.

These macros are not intended to be used on fields which require custom logic inside of their setters and getters. Just write your own in that case!

```rust
#[macro_use]
extern crate getset;

#[derive(Getters, Setters, New, Default)]
#[get(vis = "pub")]
#[get(vis = "pub", mutable)]
#[set(vis = "pub", consume)]
#[set(vis = "pub")]
#[new(vis = "pub")]
pub struct Foo<T> where T: Copy + Clone + Default {
    /// Doc comments are supported!
    /// Multiline, even.
    #[get(copy)] #[get(mutable)] #[set]
    private: T,

    /// Doc comments are supported!
    /// Multiline, even.
    public: Option<T>,

    #[set(optional)]
    optional: Option<String>,
}

fn main() {
    let mut foo: Foo<i64> = Foo::new(1).consume_set_public(3);
    assert_eq!(foo.private(), 1);
    assert_eq!(*foo.public(), Some(3));
    foo.set_private(3);
    (*foo.private_mut()) += 1;
    assert_eq!(foo.private(), 4);
    foo.set_public(4);
    assert_eq!(*foo.public(), Some(4));
    foo.set_public(None);
    assert_eq!(*foo.public(), None);
    foo.set_optional(Some("test"));
    assert_eq!(foo.optional(), &Some("test".to_string()));
    foo.set_optional(None::<&str>);
    assert_eq!(*foo.optional(), None);
}
```
```compile_fail
#[macro_use]
extern crate getset;
mod submodule {
    #[derive(Getters, Default)]
    #[get = "pub"]
    pub struct Foo {
        #[get]
        private: i32,
        public: i32,
    }
}
fn main() {
    let mut foo = submodule::Foo::default();
    assert_eq!(*foo.private(), 2);
}
```
*/

extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

mod generate;
mod parse;
mod types;

use crate::types::{GenMode, GenParams};
use proc_macro::TokenStream;
use syn::{DataStruct, DeriveInput, Visibility};

#[proc_macro_derive(Getters, attributes(get))]
pub fn getters(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = parse_macro_input!(input as DeriveInput);
    let params = GenParams {
        attribute_name: "get",
        fn_name_prefix: None,
        fn_name_suffix: None,
        global_attr: parse::global_attr(&ast.attrs, "get"),
    };

    // Build the impl
    let gen = produce(&ast, &GenMode::Get, &params);

    // Return the generated impl
    gen.into()
}

#[proc_macro_derive(MutGetters, attributes(get_mut))]
pub fn mut_getters(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: DeriveInput = syn::parse(input).expect("Couldn't parse for getters");
    let params = GenParams {
        attribute_name: "get_mut",
        fn_name_prefix: None,
        fn_name_suffix: Some("_mut"),
        global_attr: parse::global_attr(&ast.attrs, "get_mut"),
    };

    // Build the impl
    let gen = produce(&ast, &GenMode::GetMut, &params);
    // Return the generated impl
    gen.into()
}

#[proc_macro_derive(Setters, attributes(set))]
pub fn setters(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: DeriveInput = syn::parse(input).expect("Couldn't parse for setters");
    let params = GenParams {
        attribute_name: "set",
        fn_name_prefix: Some("set_"),
        fn_name_suffix: None,
        global_attr: parse::global_attr(&ast.attrs, "set"),
    };

    // Build the impl
    let gen = produce(&ast, &GenMode::Set, &params);

    // Return the generated impl
    gen.into()
}

#[proc_macro_derive(New, attributes(new))]
pub fn new(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: DeriveInput = syn::parse2(input.into()).expect("Couldn't parse for setters");

    // Build the impl
    let gen = produce_new(&ast);

    // Return the generated impl
    gen.into()
}

fn produce_new(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Is it a struct?
    if let syn::Data::Struct(DataStruct { ref fields, .. }) = ast.data {
        let generated = fields
            .iter()
            .map(|f| generate::implement_new(f))
            .collect::<Vec<_>>();

        let initialize = generated.iter().map(|(a, _)| a).collect::<Vec<_>>();
        let struct_initialize = generated.iter().map(|(_, a)| a).collect::<Vec<_>>();
        let global_attr = parse::global_attr(&ast.attrs, "new");
        let attr = global_attr.first().expect("new attribute").clone();
        let params = GenParams {
            attribute_name: "new",
            fn_name_prefix: None,
            fn_name_suffix: None,
            global_attr: global_attr,
        };

        let attributes = parse::meta(&attr, &params);
        let visibility: Option<Visibility> = attributes
            .vis
            .map(|vis| syn::parse_str(vis.as_ref()).expect("visibility"));
        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                #visibility fn new(#(#initialize)*) -> Self {
                    Self {
                        #(#struct_initialize)*
                    }
                }
            }
        }
    } else {
        // Nope. This is an Enum. We cannot handle these!
        panic!("#[derive(New)] is only defined for structs, not for enums!");
    }
}

fn produce(ast: &DeriveInput, mode: &GenMode, params: &GenParams) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Is it a struct?
    if let syn::Data::Struct(DataStruct { ref fields, .. }) = ast.data {
        let generated = fields
            .iter()
            .map(|f| generate::implement(f, mode, params))
            .collect::<Vec<_>>();

        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                #(#generated)*
            }
        }
    } else {
        // Nope. This is an Enum. We cannot handle these!
        panic!("#[derive(Getters)] is only defined for structs, not for enums!");
    }
}
