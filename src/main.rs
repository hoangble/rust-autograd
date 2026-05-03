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

fn print_value(v: &Value) {
    println!("{}: {}", v.label, v.data);
}

fn describe(v: &Value) {
    match v.op {
        Op::Leaf => println!("{} is a leaf input", v.label),
        Op::Add => println!("{} was produced by addition", v.label),
        Op::Mul => println!("{} was produced by multiplication", v.label),
        Op::Tanh => println!("{} was produced by tanh", v.label),
    }

    for child in &v.children {
        describe(child);
    }
}

fn add_data(v: &Value) -> f64 {
    v.data
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

    println!("r value: {}", r.data) // b is out of scope so r is ref-ing something out of scope
}
