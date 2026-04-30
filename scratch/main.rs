pub enum Token {
    Num(f64),
    Add,
    Mul,
    Sub,
    Div,
}

pub fn rpn(tokens: Vec<Token>) -> f64 {
    let mut stack: Vec<f64> = vec![];
    for token in tokens {
        match token {
            Token::Num(n) => stack.push(n),
            Token::Add => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            }
            Token::Mul => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b);
            }
            Token::Sub => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            }
            Token::Div => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a / b);
            }
        }
    }
    stack.pop().unwrap()
}

fn main() {
    // let result = rpn(vec![
    //     Token::Num(2.0),
    //     Token::Num(3.0),
    //     Token::Div,
    //     Token::Num(4.0),
    //     Token::Mul,
    // ]);
    // println!("{}", result); // should print 20
    // 10 - 3 → 7
    let result: f64 = rpn(vec![Token::Num(10.0), Token::Num(3.0), Token::Sub]);
    println!("{}", result);
    // 8 / 4 → 2
    let result: f64 = rpn(vec![Token::Num(8.0), Token::Num(4.0), Token::Div]);
    println!("{}", result);

    // (2 + 3) * (10 - 4) → 30
    let result: f64 = rpn(vec![
        Token::Num(2.0),
        Token::Num(3.0),
        Token::Add,
        Token::Num(10.0),
        Token::Num(4.0),
        Token::Sub,
        Token::Mul,
    ]);
    println!("{}", result);


    // 10 / (2 + 3) → 2
    let result: f64 = rpn(vec![
        Token::Num(10.0),
        Token::Num(2.0),
        Token::Num(3.0),
        Token::Add,
        Token::Div,
    ]);
    println!("{}", result);

}
