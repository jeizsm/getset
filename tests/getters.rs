#[macro_use]
extern crate getset;

#[derive(Getters)]
#[get]
pub struct Plain {
    field: usize,
}

#[derive(Getters)]
#[get(prefix = "get")]
pub struct Custom {
    field: usize,
    #[get(prefix = "get", suffix = "test")]
    second_field: usize,
}

#[test]
fn test_getters() {
    let val = Plain { field: 18 };
    assert_eq!(18, *val.field());
}

#[test]
fn test_custom_prefix() {
    let val = Custom {
        field: 20,
        second_field: 20,
    };
    assert_eq!(20, *val.get_field());
}

#[test]
fn test_custom_suffix_and_prefix() {
    let val = Custom {
        field: 20,
        second_field: 20,
    };
    assert_eq!(20, *val.get_second_field_test());
}
