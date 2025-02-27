use core::panic;
use std::{ops::{Add, Mul}};


struct Value {
    data: f64,
    children: Vec<Value>,
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
            op: String::new(),
        }
    }

    fn calculate_tanh(&self, data: f64) -> f64 {
        let x = data;
        (f64::exp(2.0*x) - 1.0)/(f64::exp(2.0*x) + 1.0)
    }

    fn tanh(self) -> Value {
        let x = self.data;
        let t = (f64::exp(2.0*x) - 1.0)/(f64::exp(2.0*x) + 1.0);

        Value::new(self.data)

    }

    fn backward_tanh(mut self, out_grad: f64){
        //leaf node
        if self.children.len() == 0 {
            return
        }

        let t = self.calculate_tanh(out_grad);
        self.grad = (1.0 - t.powf(2.0)) * out_grad;
    }

    fn backward_add(mut self, out_grad: f64) {
        //leaf node
        if self.children.len() == 0 {
            return;
        }

        self.grad += out_grad;

        match self.children.get_mut(1) {
            Some(other) => other.grad += out_grad,
            None => panic!("Was not possible to get 'other' value")
        };
    }

    fn backward_mul(mut self, out_grad: f64) {
        //leaf node
        if self.children.len() == 0 {
            return;
        }

        let other = match self.children.get_mut(1) {
            Some(other) => other,
            None => panic!("Was not possible to get 'other' value")
        };

        self.grad = other.grad * out_grad;
        other.grad = self.grad * out_grad;
    }

}

impl Add for Value{
    fn add(self, other: Self) -> Self::Output {

        let mut out = Value::new(self.data + other.data);
        
        out.children = vec![self, other];
        out.op = String::from("+");

        out
    }



    type Output = Self;
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
            let mut out = Value::new(self.data * other.data);

        out.children = vec![self, other];
        out.op = String::from("*");
        out.label = String::new();

        out


    }
}
