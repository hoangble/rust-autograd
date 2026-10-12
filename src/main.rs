use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Value {
    data: f64,
    label: String,
    children: Vec<Value>,
    op: Op,
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Tanh,
    Leaf,
}

fn main() {
    let mut a = Value {
        data: 1.0,
        label: "a".into(),
        children: vec![],
        op: Op::Leaf,
    };

    let b = Value {
        data: 2.0,
        label: "b".into(),
        children: vec![],
        op: Op::Leaf,
    };

    let mut a_grad: f64 = 2.0;
    let mut b_grad: f64 = 7.0;
    let c_grad: f64 = 5.0;

    let backward: Box<dyn Fn()> = Box::new(|| {
        a_grad += b.data * c_grad;
        b_grad += a.data * c_grad;
    });
    backward();
    // print!("{}", a_grad);
}
