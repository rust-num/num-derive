extern crate num as num_renamed;
#[macro_use]
extern crate num_derive;

use crate::num_renamed::{FromPrimitive};
use std::ops::Neg;
use std::num::Wrapping;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialOrd,
    ToPrimitive,
    FromPrimitive,
    NumOps,
    NumCast,
    One,
    Zero,
    Num,
)]
struct MyThing<T>(Wrapping<T>);

impl<T> PartialEq for MyThing<T> where Wrapping<T>: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: Neg<Output = T>> Neg for MyThing<T> where Wrapping<T>: Neg<Output = Wrapping<T>> {
    type Output = Self;
    fn neg(self) -> Self {
        MyThing(self.0.neg())
    }
}

#[test]
fn test_num_ops() {
    assert_eq!(MyThing::<u32>::from_i128(25).unwrap() + MyThing::<u32>::from_i128(10).unwrap(), MyThing::<u32>::from_i128(35).unwrap());
    assert_eq!(MyThing::<u32>::from_i128(25).unwrap() - MyThing::<u32>::from_i128(10).unwrap(), MyThing::<u32>::from_i128(15).unwrap());
    assert_eq!(MyThing::<u32>::from_i128(25).unwrap() * MyThing::<u32>::from_i128(2).unwrap(), MyThing::<u32>::from_i128(50).unwrap());
    assert_eq!(MyThing::<u32>::from_i128(25).unwrap() / MyThing::<u32>::from_i128(10).unwrap(), MyThing::<u32>::from_i128(2).unwrap());
    assert_eq!(MyThing::<u32>::from_i128(25).unwrap() % MyThing::<u32>::from_i128(10).unwrap(), MyThing::<u32>::from_i128(5).unwrap());
}
