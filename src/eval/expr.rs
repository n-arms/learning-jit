use std::collections::HashMap;

use crate::ir::expr::{Expr, Operator};

pub type Env = HashMap<usize, f32>;

pub fn evaluate(expr: &Expr, env: &Env) -> f32 {
    match expr {
        Expr::Operation { operator, operands } => {
            let first = evaluate(&operands[0], env);
            let second = evaluate(&operands[0], env);

            match operator {
                Operator::Add => first + second,
                Operator::Subtract => first - second,
                Operator::Multiply => first * second,
                Operator::Divide => first / second,
            }
        }
        Expr::Variable(variable) => env[variable],
        Expr::Number(number) => *number,
        Expr::IfPositive(if_positive) => {
            if evaluate(&if_positive.predicate, env) >= 0.0 {
                evaluate(&if_positive.consequent, env)
            } else {
                evaluate(&if_positive.alternative, env)
            }
        }
    }
}
