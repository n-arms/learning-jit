use std::collections::{hash_map::Entry, HashMap};

use crate::ir::{
    expr::Operator,
    register::{Expr, Program, Register, Value},
};

pub type Env = HashMap<Register, f32>;

pub fn evaluate(program: &Program, mut env: Env) -> f32 {
    //println!("evaluating reg with env {:?}", env);
    for statement in &program.statements {
        match statement.expr {
            Expr::Move(value) => {
                env.insert(statement.destination, evaluate_value(&value, &env));
            }
            Expr::Operation { operator, operand } => {
                let first = evaluate_value(&operand, &env);
                let entry = env.entry(statement.destination);
                assert!(matches!(entry, Entry::Occupied(..)));
                entry.and_modify(|dest| match operator {
                    Operator::Add => *dest += first,
                    Operator::Subtract => *dest -= first,
                    Operator::Multiply => *dest *= first,
                    Operator::Divide => *dest /= first,
                });
            }
            Expr::IfPositive {
                predicate,
                consequent,
                alternative,
            } => {
                let result = if evaluate_value(&predicate, &env) >= 0.0 {
                    evaluate_value(&consequent, &env)
                } else {
                    evaluate_value(&alternative, &env)
                };
                env.insert(statement.destination, result);
            }
        }
    }

    evaluate_value(&program.output, &env)
}

fn evaluate_value(value: &Value, env: &Env) -> f32 {
    match value {
        Value::Register(reg) => {
            let entry = env.get(reg);
            match entry {
                Some(value) => *value,
                None => panic!("no value in register {:?}", reg),
            }
        }
        Value::Number(number) => *number,
    }
}
