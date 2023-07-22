use anyhow::{bail, Result};
use core::panic;
use pest::pratt_parser::PrattParser;
use pest::{iterators::Pairs, Token};
use pest::{Parser, RuleType};
use std::io::{self, BufRead};

use crate::error::ParserError;

use super::{Expr, Op};

#[derive(pest_derive::Parser)]
#[grammar = "grammar/numeric_evaluator.pest"]
pub(crate) struct CalculatorParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
            .op(Op::infix(power, Right))
            .op(Op::prefix(unary_minus))
        };
}

fn parse_function(pairs: Pairs<Rule>) -> Result<Expr> {
    let mut name = String::new();
    let mut args: Vec<Box<Expr>> = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::function_name => name = String::from(pair.as_str()),
            Rule::function_args => {
                args = pair
                    .into_inner()
                    .map(|arg| parse_expr(arg.into_inner()))
                    .map(|arg| Box::new(arg.unwrap()))
                    .collect()
            }
            rule => {
                bail!(ParserError::InvalidToken(format!("{:?}", rule)))
            }
        }
    }

    if name != "" {
        Ok(Expr::Function { name, args })
    } else {
        bail!(ParserError::NoFunctionName)
    }
}

fn parse_expr(pairs: Pairs<Rule>) -> Result<Expr> {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::number => Ok(Expr::Number(primary.as_str().parse::<f64>().unwrap())),
            Rule::expr => parse_expr(primary.into_inner()),
            Rule::function => parse_function(primary.into_inner()),
            rule => bail!(ParserError::InvalidToken(format!("{:?}", rule))),
        })
        .map_infix(|lhs, op, rhs| {
            let op: Result<Op> = match op.as_rule() {
                Rule::add => Ok(Op::Add),
                Rule::subtract => Ok(Op::Subtract),
                Rule::multiply => Ok(Op::Multiply),
                Rule::divide => Ok(Op::Divide),
                Rule::modulo => Ok(Op::Modulo),
                Rule::power => Ok(Op::Power),
                rule => bail!(ParserError::InvalidOperator(format!("{:?}", rule))),
            };
            let op = op?;
            Ok(Expr::BinOp {
                lhs: Box::new(lhs?),
                op,
                rhs: Box::new(rhs?),
            })
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Ok(Expr::UnaryMinus(Box::new(rhs?))),
            rule => bail!(ParserError::InvalidToken(format!("{:?}", rule))),
        })
        .parse(pairs)
}

pub fn parse(expression: &str) -> Result<Expr> {
    let mut pairs = CalculatorParser::parse(Rule::equation, expression)?;
    parse_expr(pairs.next().unwrap().into_inner())
}

#[cfg(test)]
mod Test {
    use crate::numeric_evaluator::parse;

    #[test]
    fn can_parse_plus() {
        assert_eq!("(2+5)", parse("2+5").unwrap().to_string());
        assert_eq!("(-(2)+-(5))", parse("-2+-5").unwrap().to_string());
        assert_eq!("((2+5)+7)", parse("2+5+7").unwrap().to_string());
    }

    #[test]
    fn can_parse_minus() {
        assert_eq!("(3-7)", parse("3-7").unwrap().to_string());
        assert_eq!("(-(3)--(7))", parse("-3--7").unwrap().to_string());
        assert_eq!("((3-7)-4)", parse("3-7-4").unwrap().to_string());
    }

    #[test]
    fn can_parse_multiply() {
        assert_eq!("(6*3)", parse("6*3").unwrap().to_string());
        assert_eq!("(-(6)*-(3))", parse("-6*-3").unwrap().to_string());
        assert_eq!("((6*3)*8)", parse("6*3*8").unwrap().to_string());
    }

    #[test]
    fn can_parse_divide() {
        assert_eq!("(1/9)", parse("1/9").unwrap().to_string());
        assert_eq!("(-(1)/-(9))", parse("-1/-9").unwrap().to_string());
        assert_eq!("((1/9)/5)", parse("1/9/5").unwrap().to_string());
    }

    #[test]
    fn can_parse_modulus() {
        assert_eq!("(3%2)", parse("3%2").unwrap().to_string());
        assert_eq!("(-(3)%-(2))", parse("-3%-2").unwrap().to_string());
        assert_eq!("((3%2)%3)", parse("3%2%3").unwrap().to_string());
    }

    #[test]
    fn can_parse_power() {
        assert_eq!("(3^2)", parse("3^2").unwrap().to_string());
        assert_eq!("(-(3)^-(2))", parse("-3^-2").unwrap().to_string());
        assert_eq!("(3^(2^4))", parse("3^2^4").unwrap().to_string());
    }

    #[test]
    fn can_parse_decimal() {
        assert_eq!("3.2", parse("3.2").unwrap().to_string());
        assert_eq!("-(3.2)", parse("-3.2").unwrap().to_string());
    }

    #[test]
    fn can_parse_order_of_operations() {
        assert_eq!("(2+(4*3))", parse("2+4*3").unwrap().to_string());
        assert_eq!("((2+4)*3)", parse("(2+4)*3").unwrap().to_string());

        assert_eq!("(2-(4*3))", parse("2-4*3").unwrap().to_string());
        assert_eq!("((2-4)*3)", parse("(2-4)*3").unwrap().to_string());

        assert_eq!("(2+(4/3))", parse("2+4/3").unwrap().to_string());
        assert_eq!("((2+4)/3)", parse("(2+4)/3").unwrap().to_string());

        assert_eq!("(2-(4/3))", parse("2-4/3").unwrap().to_string());
        assert_eq!("((2-4)/3)", parse("(2-4)/3").unwrap().to_string());

        assert_eq!("(1+(2*(3^3)))", parse("1+2*3^3").unwrap().to_string());
        assert_eq!("(1+((2*3)^3))", parse("1+(2*3)^3").unwrap().to_string());
    }

    #[test]
    fn can_parse_tests_wikipedia() {
        assert_eq!(
            "(3+((4*2)/((1-5)^(2^3))))",
            parse("3+4*2/(1-5)^2^3").unwrap().to_string()
        );
        assert_eq!(
            "sin(((max(2, 3)/3)*3.1415))",
            parse("sin(max(2, 3) / 3 * 3.1415)").unwrap().to_string()
        );
    }

    #[test]
    fn can_parse_functions() {
        assert_eq!("(max(1, 2)+4)", parse("max(1, 2) + 4").unwrap().to_string());
        assert_eq!("(4+min(5, 4))", parse("4 + min(5, 4)").unwrap().to_string());
        assert_eq!(
            "(7+max(2, min(47.94, trunc(22.54))))",
            parse("7 + max(2, min(47.94, trunc(22.54)))")
                .unwrap()
                .to_string()
        );
    }
}
