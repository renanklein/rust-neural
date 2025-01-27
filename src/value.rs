use std::ops::{Add, Mul};

#[derive(Debug)]
struct Value {
    data: f64
}

impl Add for Value {
    fn add(self, rhs: Self) -> Self::Output {
        Self {data: self.data + rhs.data}
    }

    type Output = Self;
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {data: self.data * rhs.data}
    }
}
