use std::fs;

mod basic;

fn main() {
    let file = fs::read_to_string("./inputs/hello_world.bas").unwrap();
    let lines = file.lines().next().unwrap();
    let (_, (_, command)) = basic::parse_line(lines).unwrap();
    match command {
        basic::Command::Print(input) => {
            println!("{}", input);
        }
        _ => {
            panic!("Command not recognised");
        }
    };
}
