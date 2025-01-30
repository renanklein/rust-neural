use std::{ops::{Add, Mul}};

struct Value {
    data: f64,
    children: Vec<Option<Box<Value>>>,
    label: String,
    grad: f64,
    backward: fn()
}

impl Value {
    fn new (data: f64, ) -> Self {
        Value {

        }
    }
    fn tanh(self) -> Value {
        let x = self.data;
        let t = (f64::exp(2.0*x) - 1.0)/(f64::exp(2.0*x) + 1.0);

        let backward = || {

        };

        Value {data: t, children: vec![Some(Box::new(self))], label: String::from("tanh")}
    }
}

impl Add for Value{
    fn add(self, rhs: Self) -> Self::Output {
        Self {data: self.data + rhs.data, children: vec![Some(Box::new(self)), Some(Box::new(rhs))], label: String::new()}
    }

    type Output = Self;
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {data: self.data * rhs.data, children: vec![Some(Box::new(self)), Some(Box::new(rhs))], label: String::new()}
    }
}
