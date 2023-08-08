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
    pub fn print_expr(&self, indent: usize) {
        match self {
            Expr::Number(num) => println!("{:indent$}Number: {}", "", num, indent = indent),
            Expr::UnaryMinus(expr) => {
                println!("{:indent$}UnaryMinus", "", indent = indent);
                expr.print_expr(indent + 2);
            }
            Expr::BinOp { lhs, op, rhs } => {
                println!("{:indent$}BinOp: {:?}", "", op, indent = indent);
                lhs.print_expr(indent + 2);
                rhs.print_expr(indent + 2);
            }
            Expr::Function { name, args } => {
                println!("{:indent$}Function: {}", "", name, indent = indent);
                for arg in args {
                    arg.print_expr(indent + 2);
                }
            }
            Expr::Monomial {
                coefficient,
                variable,
                exponent,
            } => println!(
                "{:indent$}Monomial: {} {}^{}",
                "", coefficient, variable, exponent,
                indent = indent
            ),
        }
    }
}

impl Op {
    pub fn get_precedence(&self) -> Option<u8> {
        match self {
            Op::Add | Op::Subtract => Some(1),
            Op::Multiply | Op::Divide | Op::Modulo => Some(2),
            Op::Power => Some(3),
            Op::Equals => None,
        }
    }
}

impl Expr {
    pub fn get_bin_op(self) -> Option<(Expr, Op, Expr)> {
        match self {
            Expr::BinOp { lhs, op, rhs } => Some((*lhs, op, *rhs)),
            _ => None,
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
