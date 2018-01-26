#![deny(unused_qualifications)]
extern crate num;
#[macro_use]
extern crate num_derive;
use num::ToPrimitive;

#[derive(ToPrimitive)]
pub enum SomeEnum {
    A = 1
}

#[test]
fn test_trivial_numeric_casts() {
    assert!(SomeEnum::A.to_i64().is_some());
}
