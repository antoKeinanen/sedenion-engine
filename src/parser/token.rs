#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    UnaryMinus(Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
    Function {
        name: String,
        args: Vec<Box<Expr>>,
    },
    Monomial {
        coefficient: f64,
        variable: String,
        exponent: f64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equals,
}

impl Expr {
    pub fn optimize_expression(self) -> Expr {
        let mut old = self.clone();
        let mut latest = self.optimize_node();

        while old != latest {
            old = latest.clone();
            latest = latest.optimize_node();
        }

        latest
    }

    fn optimize_node(&self) -> Expr {
        match self {
            Expr::UnaryMinus(inner) => {
                let inner = *inner.to_owned();
                // -(-a) => a
                if let Expr::UnaryMinus(inner_inner) = inner {
                    return *inner_inner;
                }

                // -0 = 0
                if let Expr::Number(inner_n) = inner {
                    if inner_n == 0.0 {
                        return Expr::Number(0.0);
                    }
                }

                Expr::UnaryMinus(Box::new(inner.optimize_node()))
            }
            Expr::BinOp { lhs, op, rhs } => {
                let optimized_lhs = lhs.optimize_node();
                let optimized_rhs = rhs.optimize_node();

                // 0 + a = a
                if let (Expr::Number(num), Op::Add) = (&optimized_lhs, &op) {
                    if num == &0.0 {
                        return optimized_rhs;
                    }
                }

                // a + 0 = a
                if let (Expr::Number(num), Op::Add) = (&optimized_rhs, &op) {
                    if num == &0.0 {
                        return optimized_lhs;
                    }
                }

                // a - a = 0
                if let Op::Subtract = op {
                    if optimized_lhs == optimized_rhs {
                        return Expr::Number(0.0);
                    }
                }

                // 0 - a = a
                if let (Expr::Number(num), Op::Subtract) = (&optimized_lhs, &op) {
                    if num == &0.0 {
                        return optimized_rhs;
                    }
                }

                // a - 0 = a
                if let (Expr::Number(num), Op::Subtract) = (&optimized_rhs, &op) {
                    if num == &0.0 {
                        return optimized_lhs;
                    }
                }

                // 1 * a = a
                if let (Expr::Number(num), Op::Multiply) = (&optimized_lhs, &op) {
                    if num == &1.0 {
                        return optimized_rhs;
                    }
                }

                // a * 1 = a
                if let (Expr::Number(num), Op::Multiply) = (&optimized_rhs, &op) {
                    if num == &1.0 {
                        return optimized_lhs;
                    }
                }

                // 0 * a = 0
                if let (Expr::Number(num), Op::Multiply) = (&optimized_lhs, &op) {
                    if num == &0.0 {
                        return Expr::Number(0.0);
                    }
                }

                // a * 0 = 0
                if let (Expr::Number(num), Op::Multiply) = (&optimized_rhs, &op) {
                    if num == &0.0 {
                        return Expr::Number(0.0);
                    }
                }

                // a * a = a^2
                if let Op::Multiply = op {
                    if optimized_lhs == optimized_rhs {
                        if let (Expr::Monomial { .. }, Expr::Monomial { .. }) =
                            (&optimized_lhs, &optimized_rhs)
                        {
                        } else {
                            return Expr::BinOp {
                                lhs: Box::new(optimized_lhs),
                                op: Op::Power,
                                rhs: Box::new(Expr::Number(2.0)),
                            };
                        }
                    }
                }

                // a^b * a^c = a^(b+c)
                if let Op::Multiply = op {
                    if let (
                        Expr::BinOp {
                            lhs: left_lhs,
                            op: left_operator,
                            rhs: left_rhs,
                        },
                        Expr::BinOp {
                            lhs: right_lhs,
                            op: right_operator,
                            rhs: right_rhs,
                        },
                    ) = (&optimized_lhs, &optimized_rhs)
                    {
                        if let (Op::Power, Op::Power) = (left_operator, right_operator) {
                            if left_lhs == right_lhs {
                                return Expr::BinOp {
                                    lhs: Box::new(*left_lhs.to_owned()),
                                    op: Op::Power,
                                    rhs: Box::new(Expr::BinOp {
                                        lhs: left_rhs.to_owned(),
                                        op: Op::Add,
                                        rhs: right_rhs.to_owned(),
                                    }),
                                };
                            }
                        }
                    }
                }

                // a^1 = a
                if let (Expr::Number(n), Op::Power) = (&optimized_rhs, op) {
                    if n == &1.0 {
                        return optimized_lhs;
                    }
                }

                // a^-n = 1/(a^n)
                if let (Expr::UnaryMinus(inner), Op::Power) = (&optimized_rhs, op) {
                    let inner = *inner.to_owned();

                    if let Expr::Number(n) = inner {
                        return Expr::BinOp {
                            lhs: Box::new(Expr::Number(1.0)),
                            op: Op::Divide,
                            rhs: Box::new(Expr::BinOp {
                                lhs: Box::new(optimized_lhs),
                                op: Op::Power,
                                rhs: Box::new(Expr::Number(n)),
                            }),
                        };
                    }
                }

                // a / 1 = a
                if let (Expr::Number(num), Op::Divide) = (&optimized_rhs, &op) {
                    if num == &1.0 {
                        return optimized_lhs;
                    }
                }

                // a / a = 1
                if let Op::Divide = op {
                    if optimized_lhs == optimized_rhs {
                        return Expr::Number(1.0);
                    }
                }

                // aX^b + cX^d = (a+b)X^(b+d)
                if let (
                    Expr::Monomial {
                        coefficient: left_coefficient,
                        variable: left_variable,
                        exponent: left_exponent,
                    },
                    Expr::Monomial {
                        coefficient: right_coefficient,
                        variable: right_variable,
                        exponent: right_exponent,
                    },
                    Op::Add,
                ) = (&optimized_lhs, &optimized_rhs, op)
                {
                    if left_variable == right_variable && left_exponent == right_exponent {
                        return Expr::Monomial {
                            coefficient: left_coefficient + right_coefficient,
                            variable: left_variable.to_owned(),
                            exponent: left_exponent.to_owned(),
                        };
                    }
                }

                // aX^b * cX^d = (a*b)X^(b*d)
                if let (
                    Expr::Monomial {
                        coefficient: left_coefficient,
                        variable: left_variable,
                        exponent: left_exponent,
                    },
                    Expr::Monomial {
                        coefficient: right_coefficient,
                        variable: right_variable,
                        exponent: right_exponent,
                    },
                    Op::Multiply,
                ) = (&optimized_lhs, &optimized_rhs, op)
                {
                    if left_variable == right_variable {
                        return Expr::Monomial {
                            coefficient: left_coefficient * right_coefficient,
                            variable: left_variable.to_owned(),
                            exponent: left_exponent + right_exponent,
                        };
                    }
                }

                Expr::BinOp {
                    lhs: Box::new(optimized_lhs),
                    op: *op,
                    rhs: Box::new(optimized_rhs),
                }
            }
            Expr::Number(n) => Expr::Number(*n),
            Expr::Monomial { .. } => return self.clone(),
            token => todo!("Optimizing for '{token:?}' not implemented yet!"),
        }
    }
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        let mut out = String::new();
        match self {
            Expr::Number(val) => out.push_str(&val.to_string()),
            Expr::UnaryMinus(expr) => out.push_str(&format!("-({})", expr.to_string())),
            Expr::BinOp { lhs, op, rhs } => {
                let lhs = lhs.to_string();
                let rhs = rhs.to_string();
                let op = match op {
                    Op::Add => '+',
                    Op::Subtract => '-',
                    Op::Multiply => '*',
                    Op::Divide => '/',
                    Op::Modulo => '%',
                    Op::Power => '^',
                    Op::Equals => '=',
                };

                out.push_str(&format!("({lhs}{op}{rhs})"));
            }
            Expr::Function { name, args } => {
                let args = args
                    .into_iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                out.push_str(&format!("{name}({args})"));
            }
            Expr::Monomial {
                coefficient,
                variable,
                exponent,
            } => out.push_str(&format!("{coefficient}{variable}^({exponent})")),
        }
        return out;
    }
}

#[cfg(test)]
mod test {
    use crate::parser::parse;

    fn setup_single(expression: &str) -> String {
        parse(expression).unwrap().optimize_node().to_string()
    }

    fn setup_multi(expression: &str) -> String {
        parse(expression).unwrap().optimize_expression().to_string()
    }

    #[test]
    fn can_optimize_double_unary() {
        assert_eq!("25", setup_single("-(-25)"));
    }

    #[test]
    fn can_optimize_double_unary_in_expression() {
        assert_eq!("((3*5)+25)", setup_single("3*5+(-(-25))"));
    }

    #[test]
    fn can_optimize_zero_addition() {
        assert_eq!("645", setup_single("0+645"));
        assert_eq!("645", setup_single("645+0"));
    }

    #[test]
    fn can_optimize_zero_addition_in_expression() {
        assert_eq!("(55*645)", setup_single("55*(0+645)"));
        assert_eq!("(24*645)", setup_single("24*645+0"));
    }

    #[test]
    fn can_optimize_zero_subtraction() {
        assert_eq!("645", setup_single("0-645"));
        assert_eq!("645", setup_single("645-0"));
    }

    #[test]
    fn can_optimize_zero_subtraction_in_expression() {
        assert_eq!("(55*645)", setup_single("55*(0-645)"));
        assert_eq!("(24*645)", setup_single("24*645-0"));
    }

    #[test]
    fn can_optimize_double_subtraction() {
        assert_eq!("0", setup_single("112-112"));
    }

    #[test]
    fn can_optimize_double_subtraction_in_expression() {
        assert_eq!("0", setup_single("(32894/132)-(32894/132)"));
    }

    #[test]
    fn can_optimize_one_multiplication() {
        assert_eq!("645", setup_single("1*645"));
        assert_eq!("645", setup_single("645*1"));
    }

    #[test]
    fn can_optimize_one_multiplication_in_expression() {
        assert_eq!("(55*645)", setup_single("55*1*645"));
        assert_eq!("(24*645)", setup_single("24*645*1"));
    }

    #[test]
    fn can_optimize_one_division() {
        assert_eq!("645", setup_single("645/1"));
    }

    #[test]
    fn can_optimize_one_division_in_expression() {
        assert_eq!("(24*645)", setup_single("24*645/1"));
    }

    #[test]
    fn can_optimize_double_division() {
        assert_eq!("1", setup_single("112/112"));
    }

    #[test]
    fn can_optimize_double_division_in_expression() {
        assert_eq!("1", setup_single("(32894-132)/(32894-132)"));
    }

    #[test]
    fn can_optimize_double_powers() {
        assert_eq!("(3^(5+10))", setup_single("3^5*3^10"));
    }

    #[test]
    fn can_optimize_double_powers_in_expression() {
        assert_eq!(
            "(3^((3213*2)+(421*23)))",
            setup_single("3^(3213*2)*3^(421*23)")
        );
    }

    #[test]
    fn can_optimize_power_of_one() {
        assert_eq!("3", setup_single("3^1"));
    }

    #[test]
    fn can_optimize_power_of_one_in_expression() {
        assert_eq!("(3213*2)", setup_single("(3213*2)^1"));
    }

    #[test]
    fn can_optimize_power_of_negative_one() {
        assert_eq!("(1/(3^1))", setup_single("3^(-1)"));
    }

    #[test]
    fn can_optimize_power_of_negative_one_in_expression() {
        assert_eq!("(1/((3213*2)^1))", setup_single("(3213*2)^(-1)"));
    }

    #[test]
    fn can_optimize_multiple_layers() {
        assert_eq!("(1/(3213*2))", setup_multi("(3213*2)^(-1)"));
        assert_eq!("(1/0)", setup_multi("(53*88*(52-52))^(-(125/125))"));
    }

    #[test]
    fn can_optimize_monomial_plus() {
        assert_eq!("8X^(8)", setup_single("2X^8+6X^8"));
        assert_eq!("2X^(1)", setup_single("X+X"));
    }

    #[test]
    fn can_optimize_monomial_multiply() {
        assert_eq!("12X^(10)", setup_single("2X^8*6X^2"));
        assert_eq!("1X^(2)", setup_single("X*X"));
    }
}
