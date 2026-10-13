use std::cell::RefCell;
use std::rc::Rc;

use ops::Forward;
mod ops;

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

// fn compute(node: &dyn Forward) -> f64 {
//     node.forward()
// }

fn compute<T: Forward + ?Sized>(node: &T) -> f64 {
    node.forward()
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

    // let mut a_grad: f64 = 2.0;
    // let mut b_grad: f64 = 7.0;
    // let c_grad: f64 = 5.0;

    // let backward: Box<dyn Fn()> = Box::new(|| {
    //     a_grad += b.data * c_grad;
    //     b_grad += a.data * c_grad;
    // });
    // backward();

    let add_node = ops::AddNode {
        left: 0.1,
        right: 16.0,
    };
    let add_node_2 = ops::AddNode {
        left: 1.0,
        right: 2.0,
    };
    let mul_node = ops::MulNode {
        left: 0.1,
        right: 16.0,
    };
    let tanh_node = ops::TanhNode { input: 1.2 };
    let vec = vec![add_node]; //, mul_node, tanh_node];
    let result: Vec<f64> = vec.iter().map(|e| compute(e)).collect();
    result.iter().for_each(|e| println!("result {}", e));

    // testing adding for ref
    // println!("add node with reference {}", &add_node + &add_node_2);
    // println!("add node with ownership {}", add_node + add_node_2);
}
