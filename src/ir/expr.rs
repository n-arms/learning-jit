use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};

use crate::math::number::Number;

#[derive(Copy, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone)]
pub struct IfPositive {
    pub predicate: Expr,
    pub consequent: Expr,
    pub alternative: Expr,
}

#[derive(Clone)]
pub enum Expr {
    Operation {
        operator: Operator,
        operands: Box<[Expr; 2]>,
    },
    Variable(usize),
    Number(f32),
    IfPositive(Box<IfPositive>),
}

impl Number for Expr {
    fn if_positive(self, consequent: Self, alternative: Self) -> Self {
        Expr::IfPositive(Box::new(IfPositive {
            predicate: self,
            consequent,
            alternative,
        }))
    }
}

impl From<f32> for Expr {
    fn from(value: f32) -> Self {
        Self::Number(value)
    }
}

impl Add<Expr> for Expr {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Self::Output {
        Expr::Operation {
            operator: Operator::Add,
            operands: Box::new([self, rhs]),
        }
    }
}

impl Sub<Expr> for Expr {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Self::Output {
        Expr::Operation {
            operator: Operator::Subtract,
            operands: Box::new([self, rhs]),
        }
    }
}

impl Mul<Expr> for Expr {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Self::Output {
        Expr::Operation {
            operator: Operator::Multiply,
            operands: Box::new([self, rhs]),
        }
    }
}

impl Div<Expr> for Expr {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Self::Output {
        Expr::Operation {
            operator: Operator::Divide,
            operands: Box::new([self, rhs]),
        }
    }
}

impl Rem<Expr> for Expr {
    type Output = Expr;

    fn rem(self, _rhs: Expr) -> Self::Output {
        panic!("operation % unsupported for Expr")
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Operation { operator, operands } => {
                write!(f, "({:?} {:?} {:?})", &operands[0], operator, &operands[1])
            }
            Expr::IfPositive(if_positive) => if_positive.fmt(f),
            Expr::Variable(index) => write!(f, "%{}", index),
            Expr::Number(number) => write!(f, "{}", number),
        }
    }
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
        };
        write!(f, "{}", str)
    }
}

impl fmt::Debug for IfPositive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "if {:?} then {{ {:?} }} else {{ {:?} }}",
            self.predicate, self.consequent, self.alternative
        )
    }
}
