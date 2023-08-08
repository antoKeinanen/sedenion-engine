use log::trace;

use crate::parser::{Expr, Op};

fn distribute_monomials(lhs: Expr, op: Op, rhs: Expr, target: String) -> Expr {
    match (lhs.clone(), rhs.clone()) {
        (
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
        ) => {
            if left_variable == right_variable {
                let new_exponent = match op {
                    Op::Add | Op::Subtract if left_exponent == right_exponent => left_exponent,
                    Op::Multiply => left_exponent + right_exponent,
                    _ => {
                        return Expr::BinOp {
                            lhs: Box::new(lhs),
                            op,
                            rhs: Box::new(rhs),
                        }
                    }
                };

                let new_coefficient = match op {
                    Op::Add => left_coefficient + right_coefficient,
                    Op::Subtract => left_coefficient - right_coefficient,
                    Op::Multiply => left_coefficient * right_coefficient,
                    _ => left_coefficient,
                };

                trace!("Merging monomials: op:{op:?} new={new_coefficient}{left_variable}^{new_exponent}");

                return Expr::Monomial {
                    coefficient: new_coefficient,
                    variable: left_variable,
                    exponent: new_exponent,
                };
            }
        }
        (
            Expr::BinOp {
                lhs: left_lhs,
                op: left_op,
                rhs: left_rhs,
            },
            Expr::Monomial {
                coefficient: right_coefficient,
                variable: right_variable,
                exponent: right_exponent,
            },
        ) => {
            if left_op.get_precedence() == op.get_precedence() {
                // I am so sorry for the variable names...
                if let (
                    Expr::Monomial { .. },
                    Expr::Monomial {
                        variable: right_left_variable,
                        ..
                    },
                ) = (*left_lhs.clone(), *left_rhs.clone())
                {
                    if right_left_variable == target {
                        trace!("Bubbling target..");

                        return Expr::BinOp {
                            lhs: Box::new(Expr::BinOp {
                                lhs: left_lhs,
                                op: left_op,
                                rhs: Box::new(rhs),
                            }),
                            op: op,
                            rhs: left_rhs,
                        };
                    }
                }
            }
        }
        _ => {}
    }

    Expr::BinOp {
        lhs: Box::new(lhs),
        op,
        rhs: Box::new(rhs),
    }
}

impl Expr {
    pub fn optimize_expression(self, target: String) -> Expr {
        let mut old = self.clone();
        let mut latest = self.optimize_node(target.clone());

        while old != latest {
            trace!("New cycle started...");
            old = latest.clone();
            latest = latest.optimize_node(target.clone());
        }
        trace!("Done");

        latest
    }

    pub fn optimize_node(&self, target: String) -> Expr {
        self.print_expr(2);
        println!("---");
        match self.clone() {
            Expr::BinOp { lhs, op, rhs } => {
                let optimized_lhs = lhs.optimize_node(target.clone());
                let optimized_rhs = rhs.optimize_node(target.clone());

                match op {
                    Op::Add => {
                        // ======== constants ========
                        // 0 + a = a
                        if let Expr::Number(n) = optimized_lhs {
                            if n == 0.0 {
                                trace!("0+a=a");
                                return optimized_rhs;
                            }
                        }

                        // a + 0 = a
                        if let Expr::Number(n) = optimized_rhs {
                            if n == 0.0 {
                                trace!("a+0=a");
                                return optimized_lhs;
                            }
                        }

                        // ======== merges ========
                        // a + a = 2a
                        if optimized_lhs == optimized_rhs {
                            let (Expr::Monomial { .. }, Expr::Monomial { .. }) = (&optimized_lhs, &optimized_rhs) else {
                                trace!("a+a=a");
                                return Expr::BinOp {
                                    lhs: Box::new(Expr::Number(2.0)),
                                    op: Op::Multiply,
                                    rhs: Box::new(optimized_lhs),
                                };
                            };
                        }

                        // a + (-b) = a - b
                        if let Expr::UnaryMinus(inner) = optimized_rhs.clone() {
                            trace!("a+(-b)=a-b");
                            return Expr::BinOp {
                                lhs: Box::new(optimized_lhs),
                                op: Op::Subtract,
                                rhs: inner,
                            };
                        }
                    }
                    Op::Subtract => {
                        // ======== constants ========
                        // a - 0 = a
                        if let Expr::Number(n) = optimized_rhs {
                            if n == 0.0 {
                                trace!("a-0=a");
                                return optimized_lhs;
                            }
                        }

                        // 0 - a = -a
                        if let Expr::Number(n) = optimized_lhs {
                            if n == 0.0 {
                                trace!("0-a=a");
                                return Expr::UnaryMinus(Box::new(optimized_rhs));
                            }
                        }

                        // ======== merges ========
                        // a - a = 0
                        if optimized_lhs == optimized_rhs {
                            trace!("a-a=0");
                            return Expr::Number(0.0);
                        }

                        // a - (-b) = a + b
                        if let Expr::UnaryMinus(inner) = optimized_rhs.clone() {
                            trace!("a-(-b)=a+b");
                            return Expr::BinOp {
                                lhs: Box::new(optimized_lhs),
                                op: Op::Add,
                                rhs: inner,
                            };
                        }
                    }
                    Op::Multiply => {
                        // ======== constants ========
                        // 1 * a = a
                        if let Expr::Number(n) = optimized_lhs {
                            if n == 1.0 {
                                trace!("1*a=a");
                                return optimized_rhs;
                            }
                        }

                        // a * 1 = a
                        if let Expr::Number(n) = optimized_rhs {
                            if n == 1.0 {
                                trace!("a*1=a");
                                return optimized_lhs;
                            }
                        }

                        // 0 * a = 0
                        if let Expr::Number(n) = optimized_lhs {
                            if n == 0.0 {
                                trace!("0*a=0");
                                return Expr::Number(0.0);
                            }
                        }

                        // a * 0 = 0
                        if let Expr::Number(n) = optimized_rhs {
                            if n == 0.0 {
                                trace!("a*0=0");
                                return Expr::Number(0.0);
                            }
                        }

                        // ======== merges ========
                        // a * a = a^2
                        if optimized_lhs == optimized_rhs {
                            let (Expr::Monomial { .. }, Expr::Monomial { .. }) =
                                (&optimized_lhs, &optimized_rhs)
                            else {
                                trace!("a*a=a^2");
                                return Expr::BinOp {
                                    lhs: Box::new(optimized_lhs),
                                    op: Op::Power,
                                    rhs: Box::new(Expr::Number(2.0)),
                                };
                            };
                        }

                        // a^b * a^c = a^(b+c)
                        if let (
                            Expr::BinOp {
                                lhs: left_lhs,
                                op: left_op,
                                rhs: left_rhs,
                            },
                            Expr::BinOp {
                                lhs: right_lhs,
                                op: right_op,
                                rhs: right_rhs,
                            },
                        ) = (optimized_lhs.clone(), optimized_rhs.clone())
                        {
                            if left_lhs == right_lhs
                                && left_op == Op::Power
                                && right_op == Op::Power
                            {
                                trace!("a^b*a^c=a^(b+c)");
                                return Expr::BinOp {
                                    lhs: left_lhs,
                                    op: Op::Power,
                                    rhs: Box::new(Expr::BinOp {
                                        lhs: left_rhs,
                                        op: Op::Add,
                                        rhs: right_rhs,
                                    }),
                                };
                            }
                        }
                    }
                    Op::Divide => {
                        // ======== constants ========
                        // a / 1 = a
                        if let Expr::Number(n) = optimized_rhs {
                            if n == 1.0 {
                                trace!("a/1=a");
                                return optimized_lhs;
                            }
                        }

                        // ======== merges ========
                        // a / a = 1
                        if optimized_lhs == optimized_rhs {
                            trace!("a/a=1");
                            return Expr::Number(1.0);
                        }
                    }
                    Op::Power => {
                        // ======== constants ========
                        // a^0 = 1
                        if let Expr::Number(n) = optimized_rhs {
                            if n == 0.0 {
                                trace!("a^0=1");
                                return Expr::Number(1.0);
                            }
                        }

                        // a^1 = a
                        if let Expr::Number(n) = optimized_rhs {
                            if n == 1.0 {
                                trace!("a^1=a");
                                return optimized_lhs;
                            }
                        }

                        // a^(-n) = 1/a^n
                        if let Expr::UnaryMinus(inner) = optimized_rhs.clone() {
                            if let Expr::Number(n) = *inner {
                                trace!("a^(-n) = 1/a^n");
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
                    }
                    operator => todo!("{operator:?}"),
                }
                distribute_monomials(optimized_lhs, op, optimized_rhs, target.clone())
            }
            Expr::Monomial {
                coefficient,
                variable,
                exponent,
            } => {
                if coefficient == 0.0 {
                    trace!("Collapsing monomial due to coefficient");
                    return Expr::Number(0.0);
                }

                if exponent == 0.0 {
                    trace!("Collapsing monomial due to exponent");
                    return Expr::Number(1.0);
                }

                if coefficient.is_sign_negative() {
                    trace!("Applying unary to coefficient");
                    return Expr::UnaryMinus(Box::new(Expr::Monomial {
                        coefficient: coefficient.abs(),
                        variable: variable,
                        exponent: exponent,
                    }));
                }

                self.clone()
            }
            Expr::UnaryMinus(inner) => {
                let inner = inner.optimize_node(target);

                if let Expr::UnaryMinus(inner_inner) = inner {
                    trace!("Unary minuses cancel");
                    return *inner_inner;
                }

                Expr::UnaryMinus(Box::new(inner))
            }
            Expr::Number(_) => self.clone(),
            node => todo!("optimization: {node:?}"),
        }
    }

    fn apply_equation_rule(self, target: String) -> Expr {
        if let Expr::BinOp { lhs, op, rhs } = self {
            if let Op::Equals = op {
                let lhs = *lhs;
                let rhs = *rhs;

                // T + b = c => T = c - b
                if let Expr::BinOp {
                    lhs: left_lhs,
                    op: Op::Add,
                    rhs: left_rhs,
                } = lhs.clone()
                {
                    if let Expr::Monomial { variable, .. } = *left_lhs.clone() {
                        if variable == target {
                            trace!("T+b=c => T=c-b");
                            return Expr::BinOp {
                                lhs: left_lhs.clone(),
                                op: Op::Equals,
                                rhs: Box::new(Expr::BinOp {
                                    lhs: Box::new(rhs.clone()),
                                    op: Op::Subtract,
                                    rhs: left_rhs.clone(),
                                }),
                            };
                        }
                    }
                }

                // a + T = c => T = c - a
                if let Expr::BinOp {
                    lhs: left_lhs,
                    op: Op::Add,
                    rhs: left_rhs,
                } = lhs.clone()
                {
                    if let Expr::Monomial { variable, .. } = *left_rhs.clone() {
                        if variable == target {
                            trace!("a+T=c => T=c-a");
                            return Expr::BinOp {
                                lhs: left_rhs.clone(),
                                op: Op::Equals,
                                rhs: Box::new(Expr::BinOp {
                                    lhs: Box::new(rhs.clone()),
                                    op: Op::Subtract,
                                    rhs: left_lhs.clone(),
                                }),
                            };
                        }
                    }
                }

                // -(T) = a => T = -(a)
                if let Expr::UnaryMinus(inner) = lhs.clone() {
                    if let Expr::Monomial { variable, .. } = *inner.clone() {
                        if variable == target {
                            trace!("-(T) = a => T = -(a)");
                            return Expr::BinOp {
                                lhs: inner,
                                op: Op::Equals,
                                rhs: Box::new(Expr::UnaryMinus(Box::new(rhs.clone()))),
                            };
                        }
                    }
                }

                return Expr::BinOp {
                    lhs: Box::new(lhs),
                    op: Op::Equals,
                    rhs: Box::new(rhs),
                };
            } else {
                panic!("Not an equation!");
            }
        } else {
            panic!("Not an equation!");
        }
    }

    pub fn optimize_equation(self, target: String) -> Expr {
        if let Expr::BinOp { lhs, op, rhs } = self.clone() {
            if let Op::Equals = op {
                let mut old = Expr::BinOp {
                    lhs: Box::new(lhs.optimize_expression(target.clone())),
                    op: Op::Equals,
                    rhs: Box::new(rhs.optimize_expression(target.clone())),
                };

                loop {
                    let expression = old.clone().apply_equation_rule(target.clone());
                    let (lhs, op, rhs) = expression.get_bin_op().unwrap();
                    let expression = Expr::BinOp {
                        lhs: Box::new(lhs.optimize_expression(target.clone())),
                        op: Op::Equals,
                        rhs: Box::new(rhs.optimize_expression(target.clone())),
                    };

                    if expression == old {
                        return expression;
                    } else {
                        old = expression;
                    }
                }
            } else {
                panic!("Not an equation!");
            }
        } else {
            panic!("Not an equation!");
        }
    }
}
