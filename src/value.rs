use std::{ops::{Add, Mul}};

struct Value {
    data: f64,
    children: Vec<Option<Box<Value>>>,
    label: String,
    op: String,
    grad: f64,
    backward: Box<dyn FnMut() -> ()>
}

impl Value {
    fn new (data: f64) -> Self {
        Value {
            data,
            children: vec![],
            backward: Box::new(|| {}),
            grad: 0.0,
            label: String::new(),
            op: String::new()
        }
    }

    fn tanh(self) -> Value {
        let x = self.data;
        let t = (f64::exp(2.0*x) - 1.0)/(f64::exp(2.0*x) + 1.0);
        let mut out =  Value::new(self.data);

        let backward = || {

        };

        out

    }
}

impl Add for Value{
    fn add(mut self, mut rhs: Self) -> Self::Output {

        let mut out = Value::new(self.data + rhs.data);
        out.children = vec![Some(Box::new(self)), Some(Box::new(rhs))];
        out.op = String::from("+");
        let backward = || {
            self.grad = 1.0 * out.grad;
            rhs.data = 1.0 * out.grad;
        };

        out.backward = Box::new(backward);


        out

    }

    type Output = Self;
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {data: self.data * rhs.data, children: vec![Some(Box::new(self)), Some(Box::new(rhs))], label: String::new()}
    }
}
