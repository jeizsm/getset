#[macro_use]
extern crate getset;

#[derive(Setters, Default)]
#[set(consume)]
pub struct Plain {
    field: i32,
    second_field: i32,
}

#[derive(Setters, Default)]
#[set(consume, prefix = "set")]
pub struct Custom {
    field: i32,
    #[set(consume, prefix = "custom_set", suffix = "test")]
    second_field: i32,
}

#[test]
fn test_consume_setters() {
    let mut val = Plain::default();
    val = val.consume_set_field(1);
    assert_eq!(1, val.field);
}

#[test]
fn test_consume_setters_chaining() {
    let mut val = Plain::default().consume_set_field(1);
    val = val.consume_set_second_field(5);
    assert_eq!(1, val.field);
    assert_eq!(5, val.second_field);
}

#[test]
fn test_custom_prefix() {
    let mut val = Custom::default();
    val = val.set_field(1);
    assert_eq!(1, val.field);
}

#[test]
fn test_custom_prefix_and_suffix() {
    let mut val = Custom::default();
    val = val.custom_set_second_field_test(5);
    assert_eq!(5, val.second_field);
}
