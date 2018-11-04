#[macro_use]
extern crate getset;

#[derive(Getters)]
#[get(copy)]
pub struct Plain {
    field: i32,
}

#[derive(Getters)]
#[get(suffix = "", copy)]
pub struct Custom {
    field: i32,
    #[get(copy, prefix = "get", suffix = "test")]
    second_field: i32,
}

#[test]
fn test_copy_getters() {
    let val = Plain { field: 18 };
    assert_eq!(18, val.field());
}

#[test]
fn test_custom_suffix() {
    let val = Custom {
        field: 20,
        second_field: 20,
    };
    assert_eq!(20, val.field());
}

#[test]
fn test_custom_prefix_and_suffix() {
    let val = Custom {
        field: 20,
        second_field: 20,
    };
    assert_eq!(20, val.get_second_field_test());
}
