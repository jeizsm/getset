#[macro_use]
extern crate getset;

#[derive(Setters, Default)]
#[set]
pub struct Plain {
    field: usize,
    second_field: usize,
}

#[derive(Setters, Default)]
#[set(prefix = "ref_set_")]
pub struct Custom {
    field: usize,
    #[set(prefix = "custom_set_", suffix = "_test")]
    second_field: usize,
}

#[test]
fn test_setters() {
    let mut val = Plain::default();
    val.set_field(1);
    assert_eq!(1, val.field);
}

#[test]
fn test_setters_chaining() {
    let mut val = Plain::default();
    val.set_field(1).set_second_field(5);
    assert_eq!(1, val.field);
    assert_eq!(5, val.second_field);
}

#[test]
fn test_custom_prefix() {
    let mut val = Custom::default();
    val.ref_set_field(1);
    assert_eq!(1, val.field);
}

#[test]
fn test_custom_prefix_and_suffix() {
    let mut val = Custom::default();
    val.custom_set_second_field_test(5);
    assert_eq!(5, val.second_field);
}
