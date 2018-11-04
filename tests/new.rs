#[macro_use]
extern crate getset;

#[derive(New)]
#[new = "pub"]
pub struct Plain {
    field: i32,
    second_field: i32,
    _optional_field: Option<i32>,
}

#[test]
fn test_new() {
    let val = Plain::new(1, 2);
    assert_eq!(1, val.field);
    assert_eq!(2, val.second_field);
}

#[derive(New)]
#[new = "pub"]
pub struct Optional {
    optional_field: Option<i32>,
}

#[test]
fn test_optional() {
    let val = Optional::new();
    assert_eq!(None, val.optional_field);
}
