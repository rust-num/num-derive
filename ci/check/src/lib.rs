#![no_std]

use num_derive::*;

#[derive(FromPrimitive, ToPrimitive)]
pub enum ABC {
    A,
    B,
    C,
}

#[derive(PartialEq, FromPrimitive, Num, NumCast, NumOps, One, ToPrimitive, Zero)]
pub struct NewFloat(f32);
