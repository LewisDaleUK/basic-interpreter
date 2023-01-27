use nom::{
    character::{complete::i64 as cci64, streaming::one_of},
    combinator::{map, value},
    sequence::tuple,
    IResult, branch::alt, multi::many0,
};

use crate::commands::Primitive;

use super::variables::parse_int;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Divide,
    Multiply,
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            '+' => Operator::Add,
            '-' => Operator::Subtract,
            '/' => Operator::Divide,
            '*' => Operator::Multiply,
            _ => panic!("Unrecognised character"),
        }
    }
}

pub type Expression = (ExpressionTarget, Operator, ExpressionTarget);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExpressionTarget {
    Val(Primitive),
    Expression(Box<Expression>),
}

impl From<i64> for ExpressionTarget {
    fn from(value: i64) -> Self {
        ExpressionTarget::Val(Primitive::Int(value))
    }
}

impl From<Expression> for ExpressionTarget {
    fn from(value: Expression) -> Self {
        Self::Expression(Box::new(value))
    }
}

fn parse_expression_target(i: &str) -> IResult<&str, ExpressionTarget> {
    alt((
        map(parse_expression, ExpressionTarget::from),
        map(cci64, ExpressionTarget::from)
    ))(i)
}

pub fn parse_expression(i: &str) -> IResult<&str, Expression> {
    tuple((map(cci64, ExpressionTarget::from),
    map(one_of("*/+-"), Operator::from),
    map(cci64, ExpressionTarget::from)))(i)
}

pub fn parse_full_expression(i: &str) -> IResult<&str, Expression> {
    tuple((
        parse_expression_target,
        map(one_of("*/+-"), Operator::from),
        parse_expression_target,
    ))(i)
}

#[cfg(test)]
mod tests {
    use crate::commands::Primitive;

    use super::{parse_full_expression, Expression, ExpressionTarget, Operator};

    #[test]
    fn it_parses_a_simple_expression() {
        let input = "1+1";
        let expected: Expression = (
            ExpressionTarget::Val(Primitive::Int(1)),
            Operator::Add,
            ExpressionTarget::Val(Primitive::Int(1)),
        );
        let (_, result) = parse_full_expression(input).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn it_parses_a_subtraction_expression() {
        let input = "1-1";
        let expected: Expression = (
            ExpressionTarget::Val(Primitive::Int(1)),
            Operator::Subtract,
            ExpressionTarget::Val(Primitive::Int(1)),
        );
        let (_, result) = parse_full_expression(input).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn it_parses_a_left_hand_subexpression() {
        let input = "1+1+2";
        let expected: Expression = (
            ExpressionTarget::Expression(Box::new((
                ExpressionTarget::Val(Primitive::Int(1)),
                Operator::Add,
                ExpressionTarget::Val(Primitive::Int(1))
            ))),
            Operator::Add,
            ExpressionTarget::Val(Primitive::Int(2))
        );

        let (_, result) = parse_full_expression(input).unwrap();
        assert_eq!(expected, result);
    }
}
