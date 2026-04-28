struct Value {
    data: f64,
    label: String,
}

fn print_value(v: &Value) {
    println!("{}: {}", v.label, v.data);
}

fn main() {
    let v = Value {
        data: 3.0,
        label: String::from("x"),
    };
    print_value(&v);
    // print_value(&v);
}
