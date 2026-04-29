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

fn main() {
    let a = Value {
        data: 2.0,
        label: "a".into(),
        children: vec![],
        op: Op::Leaf,
    };
    let b = Value {
        data: 3.0,
        label: "b".into(),
        children: vec![],
        op: Op::Leaf,
    };
    let c = Value {
        data: 5.0,
        label: "c".into(),
        children: vec![a, b],
        op: Op::Add,
    };

    describe(&c);

    // println!("{:?}", c);
    // print_value(&v);
    //
    // let x = Value {
    // data: 2.0,
    // label: "x".into(),
    // children: vec![],
    // };
    // let d = Value {
    //     data: 4.0,
    //     label: "d".into(),
    //     children: vec![x.clone()],
    // };
    // let e = Value {
    //     data: 3.0,
    //     label: "e".into(),
    //     children: vec![x],
    // }; // x already moved!
}
