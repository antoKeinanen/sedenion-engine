use crate::parser::{Expr, Op, Optimize};

impl Optimize for Expr {
    fn optimize_expression(self) -> Expr {
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

    fn optimize_equation(self) -> Expr {
        todo!()
    }
}
