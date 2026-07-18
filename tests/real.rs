#[macro_use]
extern crate num_derive;

use num_traits::real::Real;
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
    Real,
)]
struct MyReal(f64);

impl Neg for MyReal {
    type Output = MyReal;
    fn neg(self) -> Self {
        MyReal(self.0.neg())
    }
}

#[test]
fn test_real() {
    assert_eq!(MyReal(4.0).log(MyReal(2.0)), MyReal(2.0));
}
