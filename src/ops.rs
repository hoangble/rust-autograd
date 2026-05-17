pub trait Forward {
    fn forward(&self) -> f64;
}

pub struct AddNode {
    pub left: f64,
    pub right: f64,
}
pub struct MulNode {
    pub left: f64,
    pub right: f64,
}
pub struct TanhNode {
    pub input: f64,
}
pub struct ReLUNode {
    pub input: f64,
}

impl Forward for AddNode {
    fn forward(&self) -> f64 {
        self.left + self.right
    }
}
impl Forward for MulNode {
    fn forward(&self) -> f64 {
        self.left * self.right
    }
}
impl Forward for TanhNode {
    fn forward(&self) -> f64 {
        self.input.tanh()
    }
}

impl Forward for ReLUNode {
    fn forward(&self) -> f64 {
        if self.input >= 0.0 { self.input } else { 0.0 }
    }
}

impl std::ops::Add for AddNode {
    type Output = f64 ;
    fn add(self, rhs: AddNode) -> f64  {
        self.left + rhs.left + self.right + rhs.right
    }

}
