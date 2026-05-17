mod ops;
use ops::AddNode;
use ops::Forward;
use ops::MulNode;
use ops::ReLUNode;
use ops::TanhNode;

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

fn add_data(v: &Value) -> f64 {
    v.data
}

fn compute(node: &dyn Forward) -> f64 {
    node.forward()
}

fn main() {
    let mut a = Value {
        data: 1.0,
        label: "a".into(),
        children: vec![],
        op: Op::Leaf,
    };

    let _ = add_data(&a);
    println!("{}", a.data);

    let r1 = &a; // fix from &mut -> & as there can't be two mutable ref in same scope
    let r2 = &mut a;

    let r: &Value;

    let b = Value {
        data: 2.0,
        label: "b".into(),
        children: vec![],
        op: Op::Leaf,
    };
    r = &b;

    // println!("r value: {}", r.data); // b is out of scope so r is ref-ing something out of scope

    let ops_nodes: Vec<Box<dyn Forward>> = vec![
        Box::new(AddNode {
            left: 1.0,
            right: 2.0,
        }),
        Box::new(MulNode {
            left: 1.0,
            right: 2.0,
        }),
        Box::new(TanhNode { input: -1.0 }),
        Box::new(ReLUNode { input: 1.0 }),
    ];

    for node in ops_nodes.iter() {
        println!("fwd value: {}", node.forward());
    }
    let new_node = AddNode {
        left: 1.0,
        right: 2.0,
    };
    let another_node = AddNode {
        left: 1.0,
        right: 2.0,
    };
    let sum_node = new_node + another_node;
    print!("{}", sum_node)
}
