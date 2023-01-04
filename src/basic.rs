use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, tag},
    character::{complete::u64 as ccu64, streaming::none_of},
    combinator::{map, value},
    multi::separated_list0,
    sequence::{delimited, terminated},
    IResult,
};

pub type Line = (usize, Command);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Command {
    Print(String),
    GoTo(usize),
    None,
}

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
}

impl Program {
    pub fn new(node: Node) -> Self {
        Program {
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
                    Command::Print(line) => println!("{}", line),
                    Command::GoTo(line) => iter.jump_to_line(line),
                    _ => panic!("Unrecognised command"),
                }
            };
        }
    }
}

impl From<&str> for Program {
    fn from(value: &str) -> Self {
        let (_, program) = read_program(value).unwrap();
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

fn read_string(i: &str) -> IResult<&str, String> {
    delimited(
        tag("\""),
        escaped_transform(
            none_of("\\\""),
            '\\',
            alt((value("\\", tag("\\")), value("\"", tag("\"")))),
        ),
        tag("\""),
    )(i)
}

fn match_command(i: &str) -> IResult<&str, &str> {
    alt((tag("PRINT"), tag("GO TO")))(i)
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, command): (&str, &str) = match_command(i).unwrap_or((i, ""));
    let (i, _) = tag(" ")(i)?;

    let (i, cmd) = match command {
        "PRINT" => map(read_string, Command::Print)(i)?,
        "GO TO" => map(ccu64, |line| Command::GoTo(line as usize))(i)?,
        _ => (i, Command::None),
    };

    Ok((i, cmd))
}

pub fn parse_line(line: &str) -> IResult<&str, Line> {
    let (i, line_number) = map(terminated(ccu64, tag(" ")), |l| l as usize)(line)?;
    let (i, command) = parse_command(i)?;
    Ok((i, (line_number, command)))
}

pub fn read_program(i: &str) -> IResult<&str, Program> {
    let (i, lines) = separated_list0(tag("\n"), parse_line)(i)?;
    let mut node = Node::None;

    for line in lines.iter() {
        node.push(line.clone());
    }

    Ok((i, Program::new(node)))
}

#[cfg(test)]
mod tests {
    use super::{parse_line, read_program, read_string, Command, Node};

    #[test]
    fn it_parses_a_print_command() {
        let input = "10 PRINT \"Hello, world\"";
        let expected = (10, Command::Print(String::from("Hello, world")));

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
        let expected = (10, Command::Print(String::from(r#"Hello, "world""#)));

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
            item: (10, Command::Print(String::from("Hello world"))),
            next: Box::new(Node::None),
        };
        node.push((20, Command::GoTo(10)));

        let expected = Node::Link {
            item: (10, Command::Print(String::from("Hello world"))),
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
            item: (10, Command::Print(String::from("Hello world"))),
            next: Box::new(Node::None),
        };
        node.push((20, Command::Print(String::from("I'm a second line"))));
        node.push((30, Command::Print(String::from("Still printing..."))));
        node.push((40, Command::GoTo(10)));

        let expected: Option<Node> = Some(Node::Link {
            item: (30, Command::Print(String::from("Still printing..."))),
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
            item: (10, Command::Print(String::from("Hello world"))),
            next: Box::new(Node::Link {
                item: (20, Command::GoTo(10)),
                next: Box::new(Node::None),
            }),
        };
        let expected = ("", super::Program::new(expected_node));
        let result = read_program(lines).unwrap();
        assert_eq!(expected, result);
    }
}
