use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, anychar, digit1, i64 as cci64},
    combinator::{map, not, verify},
    sequence::{preceded, terminated},
    IResult,
};

use crate::basic::Primitive;

use super::generic::{consume_line, read_string};

pub fn parse_int_variable_name(i: &str) -> IResult<&str, String> {
    map(preceded(not(digit1), alphanumeric1), String::from)(i)
}

pub fn parse_int(i: &str) -> IResult<&str, (String, Primitive)> {
    let (i, id) = parse_int_variable_name(i)?;
    let (i, _) = tag("=")(i)?;
    let (i, var) = map(cci64, Primitive::Int)(i)?;

    Ok((i, (id, var)))
}

pub fn parse_str_variable_name(i: &str) -> IResult<&str, String> {
    let (i, id) = terminated(verify(anychar, |c| c.is_alphabetic()), tag("$"))(i)?;
    let id = format!("{}$", id);
    Ok((i, id))
}

pub fn parse_str(i: &str) -> IResult<&str, (String, Primitive)> {
    let (i, id) = parse_str_variable_name(i)?;
    let (i, _) = tag("=")(i)?;
    let (i, var) = map(read_string, Primitive::String)(i)?;
    Ok((i, (id, var)))
}

pub fn parse_assignment(i: &str) -> IResult<&str, (String, Primitive)> {
    let (i, id) = alt((parse_str_variable_name, parse_int_variable_name))(i)?;
    let (i, _) = tag("=")(i)?;
    let (i, assigned_variable) = consume_line(i)?;
    Ok((
        i,
        (id, Primitive::Assignment(assigned_variable.to_string())),
    ))
}

pub fn parse_var(i: &str) -> IResult<&str, (String, Primitive)> {
    alt((parse_int, parse_str, parse_assignment))(i)
}
