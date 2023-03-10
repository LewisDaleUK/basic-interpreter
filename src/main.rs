use std::fs;

mod basic;
mod commands;
mod node;
mod parsers;

fn main() {
    let file = fs::read_to_string("./inputs/printing_program.bas").unwrap();
    let mut program = basic::Program::from(file.as_str());
    program.execute();
}
