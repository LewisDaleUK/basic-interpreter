use std::{collections::HashMap};

use nom::{bytes::complete::tag, multi::separated_list0, IResult};

use crate::{parsers, commands::{Line, Primitive, PrintOutput, Command}};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Node {
    None,
    Link { item: Line, next: Box<Node> },
}

impl Node {
    fn push(&mut self, val: Line) {
        *self = match self {
            Self::Link { item, next } => {
                next.push(val);
                Self::Link {
                    item: item.clone(),
                    next: next.clone(),
                }
            }
            Self::None => Self::Link {
                item: val,
                next: Box::new(Self::None),
            },
        }
    }

    pub fn find_line(&self, line: usize) -> Option<Node> {
        if let Self::Link { item, next } = self {
            if item.0 == line {
                Some(self.clone())
            } else {
                next.find_line(line)
            }
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    nodes: Node,
    current: Node,
    vars: HashMap<String, Primitive>,
}

impl Program {
    pub fn new(node: Node) -> Self {
        Program {
            vars: HashMap::new(),
            nodes: node.clone(),
            current: node,
        }
    }

    pub fn jump_to_line(&mut self, line: usize) {
        if let Some(node) = self.nodes.find_line(line) {
            self.current = node;
        } else {
            panic!("Cannot jump to line {}, it does not exist", line);
        }
    }

    pub fn execute(&mut self) {
        let mut iter = self.clone();

        while let Some(node) = iter.next() {
            if let Node::Link { item, next: _ } = node {
                match item.1 {
                    Command::Print(PrintOutput::Value(line)) => println!("{}", line),
                    Command::Print(PrintOutput::Variable(variable)) => {
                        println!("{}", self.vars.get(&variable).unwrap())
                    }
                    Command::GoTo(line) => iter.jump_to_line(line),
                    Command::Var((id, Primitive::Assignment(variable))) => {
                        self.vars
                            .insert(id, self.vars.get(&variable).unwrap().clone());
                    }
                    Command::Var((id, var)) => {
                        self.vars.insert(id, var);
                    }
                    Command::Comment => (),
                    _ => panic!("Unrecognised command"),
                }
            };
        }
    }

    fn read(i: &str) -> IResult<&str, Self> {
        let (i, lines) = separated_list0(tag("\n"), parsers::commands::parse_line)(i)?;
        let mut node = Node::None;

        for line in lines.iter() {
            node.push(line.clone());
        }

        Ok((i, Program::new(node)))
    }
}

impl From<&str> for Program {
    fn from(value: &str) -> Self {
        let (_, program) = Self::read(value).unwrap();
        program
    }
}

impl Iterator for Program {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.current.clone();
        match &self.current {
            Node::Link { item: _, next } => {
                self.current = *next.clone();
                Some(curr)
            }
            Node::None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::basic::PrintOutput;

    use super::{Command, Line, Node, Primitive, Program};

    use crate::parsers::{commands::parse_line, generic::read_string};

    #[test]
    fn it_parses_a_print_command() {
        let input = "10 PRINT \"Hello, world\"";
        let expected = (
            10,
            Command::Print(PrintOutput::Value(String::from("Hello, world"))),
        );

        let (_, result) = parse_line(input).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn it_reads_a_string() {
        let input = r#""Hello, \"World\"""#;
        let (_, output) = read_string(input).unwrap();
        assert_eq!(r#"Hello, "World""#, output);
    }

    #[test]
    fn it_parses_a_print_command_with_escaped_quotes() {
        let input = r#"10 PRINT "Hello, \"world\"""#;
        let expected = (
            10,
            Command::Print(PrintOutput::Value(String::from(r#"Hello, "world""#))),
        );

        let (_, result) = parse_line(input).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn it_parses_a_goto_command() {
        let input = "20 GO TO 10";
        let expected = (20, Command::GoTo(10));
        let (_, result) = parse_line(input).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn it_can_create_a_linked_list_for_a_program() {
        let mut node = Node::Link {
            item: (
                10,
                Command::Print(PrintOutput::Value(String::from("Hello world"))),
            ),
            next: Box::new(Node::None),
        };
        node.push((20, Command::GoTo(10)));

        let expected = Node::Link {
            item: (
                10,
                Command::Print(PrintOutput::Value(String::from("Hello world"))),
            ),
            next: Box::new(Node::Link {
                item: (20, Command::GoTo(10)),
                next: Box::new(Node::None),
            }),
        };
        assert_eq!(node, expected);
    }

    #[test]
    fn it_finds_a_node_by_line_number() {
        let mut node = Node::Link {
            item: (
                10,
                Command::Print(PrintOutput::Value(String::from("Hello world"))),
            ),
            next: Box::new(Node::None),
        };
        node.push((
            20,
            Command::Print(PrintOutput::Value(String::from("I'm a second line"))),
        ));
        node.push((
            30,
            Command::Print(PrintOutput::Value(String::from("Still printing..."))),
        ));
        node.push((40, Command::GoTo(10)));

        let expected: Option<Node> = Some(Node::Link {
            item: (
                30,
                Command::Print(PrintOutput::Value(String::from("Still printing..."))),
            ),
            next: Box::new(Node::Link {
                item: (40, Command::GoTo(10)),
                next: Box::new(Node::None),
            }),
        });
        let result = node.find_line(30);
        assert_eq!(expected, result);
    }

    #[test]
    fn it_reads_a_program() {
        let lines = "10 PRINT \"Hello world\"\n20 GO TO 10";
        let expected_node = Node::Link {
            item: (
                10,
                Command::Print(PrintOutput::Value(String::from("Hello world"))),
            ),
            next: Box::new(Node::Link {
                item: (20, Command::GoTo(10)),
                next: Box::new(Node::None),
            }),
        };
        let expected = Program::new(expected_node);
        let result = Program::from(lines);
        assert_eq!(expected, result);
    }

    #[test]
    fn it_parses_an_integer() {
        let line = "10 LET a=22";
        let expected: Line = (10, Command::Var((String::from("a"), Primitive::Int(22))));
        let (_, result) = parse_line(line).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn it_parses_a_many_char_integer() {
        let line = "10 LET apple=1";
        let expected: Line = (10, Command::Var((String::from("apple"), Primitive::Int(1))));
        let (_, result) = parse_line(line).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn it_fails_if_integer_var_name_starts_with_number() {
        let line = "10 LET 0apple=1";
        let result = parse_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn it_parses_a_string_variable() {
        let line = r#"10 LET a$="Hello world""#;
        let expected: Line = (
            10,
            Command::Var((
                String::from("a$"),
                Primitive::String(String::from("Hello world")),
            )),
        );
        let (_, result) = parse_line(line).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn it_fails_a_string_var_with_a_multichar_name() {
        let line = r#"10 LET asd$="Hello world""#;
        let result = parse_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn it_fails_a_string_var_with_a_numeric_name() {
        let line = r#"10 LET 0$="Hello world""#;
        let result = parse_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn it_assigns_one_variable_to_another() {
        let line = "10 LET a=b$";
        let (_, result) = parse_line(line).unwrap();
        let expected: Line = (
            10,
            Command::Var((String::from("a"), Primitive::Assignment(String::from("b$")))),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_a_print_command_with_a_variable_name() {
        let line = "10 PRINT a$";
        let (_, result) = parse_line(line).unwrap();
        let expected: Line = (
            10,
            Command::Print(PrintOutput::Variable(String::from("a$"))),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_a_comment() {
        let line = "10 REM This is an arbitrary comment";
        let (_, result) = parse_line(line).unwrap();
        let expected: Line = (10, Command::Comment);
        assert_eq!(result, expected);
    }
}
