use std::fmt;

use super::expr;

#[derive(Debug)]
pub struct Program {
    pub input: Vec<Register>,
    pub statements: Vec<Statement>,
    pub output: Value,
}

#[derive(Copy, Clone)]
pub struct Register {
    pub index: usize,
}

#[derive(Copy, Clone)]
pub enum Value {
    Register(Register),
    Number(f32),
}

pub struct Statement {
    pub destination: Register,
    pub expr: Expr,
}

pub enum Expr {
    Move(Value),
    Operation {
        operator: expr::Operator,
        operand: Value,
    },
    IfPositive {
        predicate: Value,
        consequent: Value,
        alternative: Value,
    },
}

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.index)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Register(reg) => write!(f, "{:?}", reg),
            Value::Number(num) => write!(f, "{}", num),
        }
    }
}

impl fmt::Debug for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} = {:?}", self.destination, self.expr)
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Move(value) => write!(f, "{:?}", value),
            Expr::Operation { operator, operand } => write!(f, "{:?} {:?}", operator, operand),
            Expr::IfPositive {
                predicate,
                consequent,
                alternative,
            } => write!(
                f,
                "if {:?} >= 0 then {:?} else {:?}",
                predicate, consequent, alternative
            ),
        }
    }
}
