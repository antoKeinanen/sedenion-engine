use std::f64::consts::PI;

use crate::math::{round, deg_to_rad};
use super::{parse, Expr, Op};

fn evaluate_expr(expr: Expr) -> f64 {
    match expr {
        Expr::BinOp { lhs, op, rhs } => match op {
            Op::Add => evaluate_expr(*lhs) + evaluate_expr(*rhs),
            Op::Subtract => evaluate_expr(*lhs) - evaluate_expr(*rhs),
            Op::Multiply => evaluate_expr(*lhs) * evaluate_expr(*rhs),
            Op::Divide => evaluate_expr(*lhs) / evaluate_expr(*rhs),
            Op::Modulo => (evaluate_expr(*lhs) % evaluate_expr(*rhs)).abs(),
            Op::Power => evaluate_expr(*lhs).powf(evaluate_expr(*rhs)),
        },
        Expr::Number(val) => val,
        Expr::UnaryMinus(op) => -1.0 * evaluate_expr(*op),
        Expr::Function { name, args } => match name.as_str() {
            "cos" => {
                assert_eq!(args.len(), 1);
                let arg = args.iter().next().unwrap().to_owned();
                deg_to_rad(evaluate_expr(*arg)).cos()
            }
            "sin" => {
                assert_eq!(args.len(), 1);
                let arg = args.iter().next().unwrap().to_owned();
                deg_to_rad(evaluate_expr(*arg)).sin()
            }
            "tan" => {
                assert_eq!(args.len(), 1);
                let arg = args.iter().next().unwrap().to_owned();
                deg_to_rad(evaluate_expr(*arg)).tan()
            }
            "floor" => {
                assert_eq!(args.len(), 1);
                let arg = args.iter().next().unwrap().to_owned();
                evaluate_expr(*arg).floor()
            }
            "ceil" => {
                assert_eq!(args.len(), 1);
                let arg = args.iter().next().unwrap().to_owned();
                evaluate_expr(*arg).ceil()
            }
            "round" => {
                assert_eq!(args.len(), 1);
                let arg = args.iter().next().unwrap().to_owned();
                evaluate_expr(*arg).round()
            }
            "trunc" => {
                assert_eq!(args.len(), 1);
                let arg = args.iter().next().unwrap().to_owned();
                evaluate_expr(*arg).trunc()
            }
            "fract" => {
                assert_eq!(args.len(), 1);
                let arg = args.iter().next().unwrap().to_owned();
                evaluate_expr(*arg).fract()
            }
            "sqrt" => {
                assert_eq!(args.len(), 1);
                let arg = args.iter().next().unwrap().to_owned();
                evaluate_expr(*arg).sqrt()
            }
            "pow" => {
                assert_eq!(args.len(), 2);
                let mut args = args.iter();
                let arg1 = args.next().unwrap().to_owned();
                let arg2 = args.next().unwrap().to_owned();
                evaluate_expr(*arg1).powf(evaluate_expr(*arg2))
            }
            "min" => {
                assert_eq!(args.len(), 2);
                let mut args = args.iter();
                let arg1 = args.next().unwrap().to_owned();
                let arg2 = args.next().unwrap().to_owned();
                evaluate_expr(*arg1).min(evaluate_expr(*arg2))
            }
            "max" => {
                assert_eq!(args.len(), 2);
                let mut args = args.iter();
                let arg1 = args.next().unwrap().to_owned();
                let arg2 = args.next().unwrap().to_owned();
                evaluate_expr(*arg1).max(evaluate_expr(*arg2))
            }
            "parse_failure_function" => panic!("Parse error: failed to parse function"),
            name => panic!("Syntax error: unknown function '{name}'"),
        },
        _ => todo!(),
    }
}

pub fn evaluate(expression: &str) -> f64 {
    round(evaluate_expr(parse(expression)), 15)
}

#[cfg(test)]
mod Test {
    use crate::numeric_evaluator::evaluate;

    #[test]
    fn can_eval_plus() {
        assert_eq!(7.0, evaluate("2+5"));
        assert_eq!(-7.0, evaluate("-2+-5"));
        assert_eq!(14.0, evaluate("2+5+7"));
    }

    #[test]
    fn can_eval_minus() {
        assert_eq!(-4.0, evaluate("3-7"));
        assert_eq!(4.0, evaluate("-3--7"));
        assert_eq!(-8.0, evaluate("3-7-4"));
    }

    #[test]
    fn can_eval_multiply() {
        assert_eq!(18.0, evaluate("6*3"));
        assert_eq!(18.0, evaluate("-6*-3"));
        assert_eq!(144.0, evaluate("6*3*8"));
    }

    #[test]
    fn can_eval_divide() {
        assert_eq!(0.1, evaluate("1/10"));
        assert_eq!(0.1, evaluate("-1/-10"));
        assert_eq!(0.02, evaluate("1/10/5"));
    }

    #[test]
    fn can_eval_modulus() {
        assert_eq!(1.0, evaluate("3%2"));
        assert_eq!(1.0, evaluate("-3%-2"));
        assert_eq!(1.0, evaluate("3%2%3"));
    }

    #[test]
    fn can_eval_power() {
        assert_eq!(9.0, evaluate("3^2"));
        assert_eq!(0.0625, evaluate("-4^-2"));
        assert_eq!(43046721.0, evaluate("3^2^4"));
    }

    #[test]
    fn can_eval_decimal() {
        assert_eq!(3.2, evaluate("3.2"));
        assert_eq!(-3.2, evaluate("-3.2"));
    }

    #[test]
    fn can_eval_order_of_operations() {
        assert_eq!(14.0, evaluate("2+4*3"));
        assert_eq!(18.0, evaluate("(2+4)*3"));

        assert_eq!(-10.0, evaluate("2-4*3"));
        assert_eq!(-6.0, evaluate("(2-4)*3"));

        assert_eq!(4.0, evaluate("2+4/2"));
        assert_eq!(3.0, evaluate("(2+4)/2"));

        assert_eq!(0.0, evaluate("2-4/2"));
        assert_eq!(-1.0, evaluate("(2-4)/2"));

        assert_eq!(55.0, evaluate("1+2*3^3"));
        assert_eq!(217.0, evaluate("1+(2*3)^3"));
    }

    #[test]
    fn can_eval_tests_wikipedia() {
        assert_eq!(3.0001220703125, evaluate("3+4*2/(1-5)^2^3"));
    }

    #[test]
    fn can_eval_functions() {
        assert_eq!(0.5, evaluate("cos(60)"));
        assert_eq!(0.5, evaluate("sin(30)"));
        assert_eq!(1.0, evaluate("tan(45)"));
        assert_eq!(1.0, evaluate("tan(45)"));
        assert_eq!(4.0, evaluate("floor(4.5)"));
        assert_eq!(5.0, evaluate("ceil(4.5)"));
        assert_eq!(5.0, evaluate("round(4.6)"));
        assert_eq!(1.0, evaluate("trunc(1.128)"));
        assert_eq!(0.128, evaluate("fract(1.128)"));
        assert_eq!(2.0, evaluate("sqrt(4)"));
        assert_eq!(16.0, evaluate("pow(4, 2)"));
        assert_eq!(2.0, evaluate("min(4, 2)"));
        assert_eq!(4.0, evaluate("max(4, 2)"));

        assert_eq!(6.0, evaluate("max(1, 2) + 4"));
        assert_eq!(8.0, evaluate("4 + min(5, 4)"));
        assert_eq!(29.0, evaluate("7 + max(2, min(47.94, trunc(22.54)))"));
    }
}
