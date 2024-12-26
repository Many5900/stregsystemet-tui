use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Money(i32);

impl Money {
    pub fn new(cents: i32) -> Self {
        Self(cents)
    }

    pub fn kroner(&self) -> i32 {
        self.0 / 100
    }

    pub fn cents(&self) -> i32 {
        self.0.abs() % 100
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{:02} DKK", self.kroner(), self.cents())
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Money(self.0 + other.0)
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Money(self.0 - other.0)
    }
}

impl Mul<u32> for Money {
    type Output = Self;

    fn mul(self, other: u32) -> Self {
        Money(self.0 * other as i32)
    }
}

impl Mul<i32> for Money {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Money(self.0 * other)
    }
}

impl Div<u32> for Money {
    type Output = Self;

    fn div(self, other: u32) -> Self {
        Money(self.0 / other as i32)
    }
}

impl Div<i32> for Money {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        Money(self.0 / other)
    }
}

impl PartialEq<i32> for Money {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<i32> for Money {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl From<i32> for Money {
    fn from(cents: i32) -> Self {
        Money::new(cents)
    }
}
