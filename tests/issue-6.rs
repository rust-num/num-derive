#![deny(trivial_numeric_casts)]
extern crate num;
#[macro_use]
extern crate num_derive;

#[derive(FromPrimitive)]
pub enum SomeEnum {
    A = 1
}

#[test]
fn test_trivial_numeric_casts() {
    use num::FromPrimitive;
    assert!(SomeEnum::from_u64(1).is_some());
    assert!(SomeEnum::from_i64(-1).is_none());
}
