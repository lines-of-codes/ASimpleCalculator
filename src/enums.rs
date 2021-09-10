#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operators {
    None,
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    CE,
    C,
    Delete,
}

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Number(i32),
    Operator(Operators),
    Dot,
}