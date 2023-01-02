use nom::{
    bytes::complete::{tag, take_until},
    character::complete::u32 as ccu32,
    combinator::map,
    sequence::{delimited, terminated, tuple},
    IResult,
};

pub type Line<'a> = (u32, Command<'a>);

#[derive(Debug, PartialEq, Eq)]
pub enum Command<'a> {
    Print(&'a str),
    None,
}

fn read_string(i: &str) -> IResult<&str, &str> {
    take_until("\"")(i)
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, (command, _)) = tuple((take_until(" "), tag(" ")))(i)?;

    let (i, cmd) = match command {
        "PRINT" => map(delimited(tag("\""), read_string, tag("\"")), Command::Print)(i)?,
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
        let expected = (10, super::Command::Print("Hello, world"));

        let (_, result) = super::parse_line(input).unwrap();
        assert_eq!(expected, result);
    }
}
