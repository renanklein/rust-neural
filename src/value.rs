use std::{ops::{Add, Mul}};


struct Value {
    data: f64,
    children: Vec<Option<Box<Value>>>,
    label: String,
    op: String,
    grad: f64
 }

impl Value {
    fn new (data: f64) -> Self {
        Value {
            data,
            children: vec![],
            grad: 0.0,
            label: String::new(),
            op: String::new()
        }
    }

    fn tanh(self) -> Value {
        let x = self.data;
        let t = (f64::exp(2.0*x) - 1.0)/(f64::exp(2.0*x) + 1.0);
        let mut out =  Value::new(self.data);

        out

    }
    fn backward_add(out_grad: f64) -> f64 {
        1.0 * out_grad
    }
}

impl Add for Value{
    fn add(mut self, mut rhs: Self) -> Self::Output {

        let mut out = Value::new(self.data + rhs.data);
        

        out.children = vec![Some(Box::new(self)), Some(Box::new(rhs))];
        out.op = String::from("+");

        out


    }



    type Output = Self;
}

impl Mul for Value {
    type Output = Self;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
            let mut out = Value::new(self.data * rhs.data);

        out.children = vec![Some(Box::new(self)), Some(Box::new(rhs))];
        out.op = String::from("*");
        out.label = String::new();

        out


    }
}
