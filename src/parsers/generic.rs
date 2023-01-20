use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, tag, take_while},
    character::complete::none_of,
    combinator::value,
    sequence::delimited,
    IResult,
};

// Take everything until it hits a newline, if it does
pub fn consume_line(i: &str) -> IResult<&str, &str> {
    take_while(|c| c != '\n')(i)
}

pub fn read_string(i: &str) -> IResult<&str, String> {
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
