use std::fmt::Display;

pub type Line = (usize, Command);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Command {
    Print(PrintOutput),
    GoTo(usize),
    Var((String, Primitive)),
    Comment,
    None,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Primitive {
    Int(i64),
    String(String),
    Assignment(String),
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Int(i) => write!(f, "{}", i),
            Primitive::String(s) => write!(f, "{}", s),
            Primitive::Assignment(a) => write!(f, "Assigment from {}", a),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PrintOutput {
    Value(String),
    Variable(String),
}
