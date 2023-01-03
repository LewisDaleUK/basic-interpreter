use std::fs;

mod basic;

fn main() {
    let file = fs::read_to_string("./inputs/simple_program.bas").unwrap();
    let (_, mut program) = basic::read_program(&file).unwrap();
    program.execute();
}
