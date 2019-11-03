#[macro_use]
extern crate num_derive;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    ToPrimitive,
    FromPrimitive,
)]
#[num_traits = "num"]
#[repr(u8)]
enum Rgb {
    Red = 0,
    Green = 1,
    Black = 2,
}
