use std::collections::HashMap;

use crate::ir::expr::{Expr, Operator};

pub type Env = HashMap<usize, f32>;

pub fn evaluate_rec(expr: &Expr, env: &Env) -> f32 {
    match expr {
        Expr::Operation { operator, operands } => {
            let first = evaluate_rec(&operands[0], env);
            let second = evaluate_rec(&operands[1], env);

            println!(
                "{:?} = {}, {:?} = {}",
                operands[0], first, operands[1], second
            );

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
            if evaluate_rec(&if_positive.predicate, env) >= 0.0 {
                evaluate_rec(&if_positive.consequent, env)
            } else {
                evaluate_rec(&if_positive.alternative, env)
            }
        }
    }
}

pub fn evaluate(expr: &Expr, env: &Env) -> f32 {
    println!("evaluating expr with env {:?}", env);
    evaluate_rec(expr, env)
}
