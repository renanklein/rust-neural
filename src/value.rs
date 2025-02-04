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

    fn calculate_tanh(&self, data: f64) -> f64 {
        let x = data;
        (f64::exp(2.0*x) - 1.0)/(f64::exp(2.0*x) + 1.0)
    }

    fn tanh(self) -> Value {
        let x = self.data;
        let t = (f64::exp(2.0*x) - 1.0)/(f64::exp(2.0*x) + 1.0);
        let out =  Value::new(self.data);

        out

    }

    fn backward_tanh(mut self, out_grad: f64){
        let t = self.calculate_tanh(out_grad);
        self.grad = (1.0 - t.powf(2.0)) * out_grad;
    }

    fn backward_add(mut self, out_grad: f64) -> f64 {
        self.data = 1.0 * out_grad;
        1.0 * out_grad
    }

    fn backward_mul(mut self, other_grad: f64, out_grad: f64) -> f64 {
        self.grad = other_grad * out_grad;
        self.grad * out_grad
    }

}

impl Add for Value{
    fn add(self, other: Self) -> Self::Output {

        let mut out = Value::new(self.data + other.data);
        

        out.children = vec![Some(Box::new(self)), Some(Box::new(other))];
        out.op = String::from("+");

        out


    }



    type Output = Self;
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
            let mut out = Value::new(self.data * other.data);

        out.children = vec![Some(Box::new(self)), Some(Box::new(other))];
        out.op = String::from("*");
        out.label = String::new();

        out


    }
}
