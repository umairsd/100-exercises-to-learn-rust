// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SaturatingU16 {
    value: u16,
}

impl SaturatingU16 {
    fn new(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

// MARK: - From

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16::new(value)
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16::new(*value)
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16::new(value.into())
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16::new((*value).into())
    }
}

// MARK: - Add

impl SaturatingU16 {
    fn overflow_add(lhs: &u16, rhs: &u16) -> SaturatingU16 {
        let result = match (*lhs).checked_add(*rhs) {
            None => u16::MAX,
            Some(v) => v,
        };
        SaturatingU16::new(result)
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: u16) -> Self {
        SaturatingU16::overflow_add(&self.value, &rhs)
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self {
        SaturatingU16::overflow_add(&self.value, rhs)
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: SaturatingU16) -> Self {
        SaturatingU16::overflow_add(&self.value, &rhs.value)
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self {
        SaturatingU16::overflow_add(&self.value, &(*rhs).value)
    }
}
