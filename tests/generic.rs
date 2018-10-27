#[macro_use]
extern crate getset;

#[derive(Getters, Setters, Default)]
#[get]
#[set]
pub struct Generic<T: Copy + Clone + Default> {
    field: T,
}

#[test]
fn test_generic_field() {
    let mut val = Generic::default();
    val.field();
    val.set_field(1);
}

#[derive(Getters, Setters, Default)]
#[get]
#[set]
pub struct Where<T>
where
    T: Copy + Clone + Default,
{
    field: T,
}

#[test]
fn test_where() {
    let mut val = Where::default();
    val.field();
    val.set_field(1);
}
