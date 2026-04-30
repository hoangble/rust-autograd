pub enum Token {
    Num(f64),
    Add,
    Mul,
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
        }
    }
    stack.pop().unwrap()
}
