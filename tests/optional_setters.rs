#[macro_use]
extern crate getset;

#[derive(Setters, Default)]
#[set(optional)]
pub struct Plain {
    field: Option<i32>,
    #[set(optional, consume)]
    second_field: Option<i32>,
}

#[test]
fn test_optional_setters() {
    let mut val = Plain::default();
    val.set_field(1i8.into());
    assert_eq!(Some(1), val.field);
    val.set_field(None::<i32>);
    assert_eq!(None, val.field);
    val.set_field(Some(1i8));
    assert_eq!(Some(1), val.field);
}


#[test]
fn test_consume_optional_setters() {
    let val = Plain::default().consume_set_second_field(1u8.into());
    assert_eq!(Some(1), val.second_field);
}
