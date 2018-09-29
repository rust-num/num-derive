extern crate num as num_renamed;
#[macro_use]
extern crate num_derive;

use num_renamed::{FromPrimitive, ToPrimitive, NumCast, One, Zero};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    ToPrimitive,
    FromPrimitive,
    NumOps,
    NumCast,
    One,
    Zero,
)]
struct MyFloat(f64);

#[test]
fn test_from_primitive() {
    assert_eq!(MyFloat::from_u32(25), Some(MyFloat(25.0)));
}

#[test]
fn test_to_primitive() {
    assert_eq!(MyFloat(25.0).to_u32(), Some(25));
}

#[test]
fn test_num_ops() {
    assert_eq!(MyFloat(25.0) + MyFloat(10.0), MyFloat(35.0));
    assert_eq!(MyFloat(25.0) - MyFloat(10.0), MyFloat(15.0));
    assert_eq!(MyFloat(25.0) * MyFloat(2.0), MyFloat(50.0));
    assert_eq!(MyFloat(25.0) / MyFloat(10.0), MyFloat(2.5));
    assert_eq!(MyFloat(25.0) % MyFloat(10.0), MyFloat(5.0));
}

#[test]
fn test_num_cast() {
    assert_eq!(<MyFloat as NumCast>::from(25u8), Some(MyFloat(25.0)));
}

#[test]
fn test_zero() {
    assert_eq!(MyFloat::zero(), MyFloat(0.0));
}

#[test]
fn test_one() {
    assert_eq!(MyFloat::one(), MyFloat(1.0));
}
