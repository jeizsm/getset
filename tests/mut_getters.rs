#[macro_use]
extern crate getset;

#[derive(MutGetters)]
#[get_mut]
pub struct Deprecated {
    field: i32,
}

#[derive(Getters)]
#[get(mutable)]
pub struct Plain {
    field: i32,
}

#[derive(Getters)]
#[get(suffix = "", mutable)]
pub struct Custom {
    field: i32,
    #[get(mutable, prefix = "get", suffix = "test")]
    second_field: i32,
}

#[test]
fn test_deprecated_mutable_getters() {
    let mut val = Deprecated { field: 18 };
    *val.field_mut() += 1;
    assert_eq!(19, *val.field_mut());
}

#[test]
fn test_mutable_getters() {
    let mut val = Plain { field: 18 };
    *val.field_mut() += 1;
    assert_eq!(19, *val.field_mut());
}

#[test]
fn test_custom_suffix() {
    let mut val = Custom {
        field: 20,
        second_field: 20,
    };
    *val.field() += 1;
    assert_eq!(21, *val.field());
}

#[test]
fn test_custom_prefix_and_suffix() {
    let mut val = Custom {
        field: 20,
        second_field: 20,
    };
    *val.get_second_field_test() += 1;
    assert_eq!(21, *val.get_second_field_test());
}
