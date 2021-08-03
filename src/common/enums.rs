#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Num(f64),
    Op(Operator),
    LParen,
    RParen
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Sci,
    Mod
}