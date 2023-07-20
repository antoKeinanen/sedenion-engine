#[derive(Debug)]
pub enum Expr {
    Number(f64),
    UnaryMinus(Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
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
        }
        return out;
    }
}
