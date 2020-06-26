extern crate num as num_renamed;
#[macro_use]
extern crate num_derive;

use crate::num_renamed::{Float, FromPrimitive, Num, NumCast, One, ToPrimitive, Zero};
use std::ops::Neg;

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
    Num,
    Float,
)]
struct MyThing<T: Cake>(T)
where
    T: Lie;

trait Cake {}
trait Lie {}

impl Cake for f32 {}
impl Lie for f32 {}

impl<T: Neg<Output = T> + Cake + Lie> Neg for MyThing<T> {
    type Output = Self;
    fn neg(self) -> Self {
        MyThing(self.0.neg())
    }
}

#[test]
fn test_from_primitive() {
    assert_eq!(MyThing::from_u32(25), Some(MyThing(25.0)));
}

#[test]
fn test_from_primitive_128() {
    assert_eq!(
        MyThing::from_i128(std::i128::MIN),
        Some(MyThing((-2.0).powi(127)))
    );
}

#[test]
fn test_to_primitive() {
    assert_eq!(MyThing(25.0).to_u32(), Some(25));
}

#[test]
fn test_to_primitive_128() {
    let f: MyThing<f32> = MyThing::from_f32(std::f32::MAX).unwrap();
    assert_eq!(f.to_i128(), None);
    assert_eq!(f.to_u128(), Some(0xffff_ff00_0000_0000_0000_0000_0000_0000));
}

#[test]
fn test_num_ops() {
    assert_eq!(MyThing(25.0) + MyThing(10.0), MyThing(35.0));
    assert_eq!(MyThing(25.0) - MyThing(10.0), MyThing(15.0));
    assert_eq!(MyThing(25.0) * MyThing(2.0), MyThing(50.0));
    assert_eq!(MyThing(25.0) / MyThing(10.0), MyThing(2.5));
    assert_eq!(MyThing(25.0) % MyThing(10.0), MyThing(5.0));
}

#[test]
fn test_num_cast() {
    assert_eq!(<MyThing<f32> as NumCast>::from(25u8), Some(MyThing(25.0)));
}

#[test]
fn test_zero() {
    assert_eq!(MyThing::zero(), MyThing(0.0));
}

#[test]
fn test_one() {
    assert_eq!(MyThing::one(), MyThing(1.0));
}

#[test]
fn test_num() {
    assert_eq!(MyThing::from_str_radix("25", 10).ok(), Some(MyThing(25.0)));
}

#[test]
fn test_float() {
    assert_eq!(MyThing(4.0).log(MyThing(2.0)), MyThing(2.0));
}
