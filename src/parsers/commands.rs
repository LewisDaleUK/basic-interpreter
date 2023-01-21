use nom::{
    branch::alt, bytes::complete::tag, character::complete::u64 as ccu64, combinator::map,
    sequence::terminated, IResult,
};

use crate::basic::{Command, Line, PrintOutput};

use super::{generic, variables};

pub fn match_command(i: &str) -> IResult<&str, &str> {
    alt((tag("PRINT"), tag("GO TO"), tag("LET"), tag("REM")))(i)
}

pub fn parse_print_command(i: &str) -> IResult<&str, PrintOutput> {
    alt((
        map(
            alt((
                variables::parse_str_variable_name,
                variables::parse_int_variable_name,
            )),
            PrintOutput::Variable,
        ),
        map(generic::read_string, PrintOutput::Value),
    ))(i)
}

pub fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, command): (&str, &str) = match_command(i).unwrap_or((i, ""));
    let (i, _) = tag(" ")(i)?;

    let (i, cmd) = match command {
        "PRINT" => map(parse_print_command, Command::Print)(i)?,
        "GO TO" => map(ccu64, |line| Command::GoTo(line as usize))(i)?,
        "LET" => map(variables::parse_var, Command::Var)(i)?,
        "REM" => {
            let (i, _) = generic::consume_line(i)?;
            (i, Command::Comment)
        }
        _ => (i, Command::None),
    };

    Ok((i, cmd))
}

pub fn parse_line(line: &str) -> IResult<&str, Line> {
    let (i, line_number) = map(terminated(ccu64, tag(" ")), |l| l as usize)(line)?;
    let (i, command) = parse_command(i)?;
    Ok((i, (line_number, command)))
}
