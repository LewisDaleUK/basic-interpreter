use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, tag, take_until},
    character::{
        complete::{u32 as ccu32},
        streaming::none_of,
    },
    combinator::{map, value},
    sequence::{delimited, terminated, tuple},
    IResult,
};

pub type Line = (u32, Command);

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Print(String),
    None,
}

fn read_string(i: &str) -> IResult<&str, String> {
    // take_until("\"")(i)
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

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, (command, _)) = tuple((take_until(" "), tag(" ")))(i)?;

    let (i, cmd) = match command {
        "PRINT" => map(read_string, Command::Print)(i)?,
        _ => (i, Command::None),
    };

    Ok((i, cmd))
}

pub fn parse_line(line: &str) -> IResult<&str, Line> {
    let (i, line_number) = terminated(ccu32, tag(" "))(line)?;
    let (i, command) = parse_command(i)?;
    Ok((i, (line_number, command)))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_parses_a_print_command() {
        let input = "10 PRINT \"Hello, world\"";
        let expected = (10, super::Command::Print(String::from("Hello, world")));

        let (_, result) = super::parse_line(input).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn it_reads_a_string() {
        let input = r#""Hello, \"World\"""#;
        let (_, output) = super::read_string(input).unwrap();
        assert_eq!(r#"Hello, "World""#, output);
    }

    #[test]
    fn it_parses_a_print_command_with_escaped_quotes() {
        let input = r#"10 PRINT "Hello, \"world\"""#;
        let expected = (10, super::Command::Print(String::from(r#"Hello, "world""#)));

        let (_, result) = super::parse_line(input).unwrap();
        assert_eq!(expected, result);
    }
}
