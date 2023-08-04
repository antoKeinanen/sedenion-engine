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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
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
                        return Expr::BinOp {
                            lhs: Box::new(optimized_lhs),
                            op: Op::Power,
                            rhs: Box::new(Expr::Number(2.0)),
                        };
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

                Expr::BinOp {
                    lhs: Box::new(optimized_lhs),
                    op: *op,
                    rhs: Box::new(optimized_rhs),
                }
            }
            Expr::Number(n) => Expr::Number(*n),
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
        }
        return out;
    }
}
