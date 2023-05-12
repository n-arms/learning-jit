use std::collections::HashMap;

use crate::ir::register::{Expr, Program, Register, Statement, Value};

#[derive(Debug)]
pub struct LiveRange {
    pub start: usize,
    pub end: usize,
    pub register: Register,
}

pub fn live_ranges(program: &Program) -> Vec<LiveRange> {
    let mut ranges = HashMap::new();

    for reg in &program.input {
        ranges.insert(
            *reg,
            LiveRange {
                start: 0,
                end: 0,
                register: *reg,
            },
        );
    }

    for (index, statement) in program.statements.iter().enumerate() {
        let mut registers = vec![statement.destination];

        match statement.expr {
            Expr::Move(Value::Register(register)) => registers.push(register),
            Expr::Operation {
                operand: Value::Register(register),
                ..
            } => registers.push(register),
            Expr::IfPositive {
                predicate,
                consequent,
                alternative,
            } => {
                for value in [predicate, consequent, alternative] {
                    if let Value::Register(register) = value {
                        registers.push(register);
                    }
                }
            }
            _ => {}
        }

        for register in registers {
            if let Some(range) = ranges.get_mut(&register) {
                range.end = index;
            } else {
                ranges.insert(
                    register,
                    LiveRange {
                        start: index,
                        end: index,
                        register,
                    },
                );
            }
        }
    }

    ranges.into_values().collect()
}

fn apply_allocation(program: &mut Program, allocation: &HashMap<Register, Register>) {
    for i in 0..program.input.len() {
        if let Some(new_reg) = allocation.get(&program.input[i]) {
            program.input[i] = *new_reg;
        }
    }

    for statement in program.statements.iter_mut() {
        statement.destination = allocation
            .get(&statement.destination)
            .copied()
            .unwrap_or(statement.destination);
        match &mut statement.expr {
            Expr::Move(Value::Register(register)) => {
                *register = allocation.get(register).copied().unwrap_or(*register);
            }
            Expr::Operation {
                operand: Value::Register(register),
                ..
            } => {
                *register = allocation.get(register).copied().unwrap_or(*register);
            }
            Expr::IfPositive {
                predicate,
                consequent,
                alternative,
            } => {
                for value in [predicate, consequent, alternative] {
                    if let Value::Register(register) = value {
                        *register = allocation.get(register).copied().unwrap_or(*register);
                    }
                }
            }
            _ => {}
        }
    }

    if let Value::Register(register) = &mut program.output {
        *register = allocation.get(register).copied().unwrap_or(*register);
    }
}

struct RegisterPool {
    unused: usize,
    freed: Vec<Register>,
}

impl RegisterPool {
    fn new() -> Self {
        Self {
            unused: 0,
            freed: Vec::new(),
        }
    }

    fn free(&mut self, register: Register) {
        self.freed.push(register);
    }

    fn get(&mut self) -> Register {
        if let Some(register) = self.freed.pop() {
            register
        } else {
            let index = self.unused;
            self.unused += 1;
            Register { index }
        }
    }
}

/// Return the number of registers the new program requires
pub fn realloc(program: &mut Program) -> usize {
    let mut ranges = live_ranges(program);
    ranges.sort_by_key(|range| range.start);

    let mut active_ranges: Vec<LiveRange> = Vec::new();
    let mut free_registers = RegisterPool::new();
    let mut substitution = HashMap::new();

    // iterate over the ranges in increasing starting order.
    // for each range, GC all the now dead ranges, then choose a new register
    for current_range in ranges {
        active_ranges.retain(|active_range| {
            let old = active_range.end < current_range.start;

            if old {
                free_registers.free(substitution[&active_range.register]);
            }
            !old
        });

        let new_reg = free_registers.get();
        substitution.insert(current_range.register, new_reg);
        active_ranges.push(current_range);
    }
    apply_allocation(program, &substitution);

    substitution
        .into_values()
        .map(|x| x.index + 1)
        .max()
        .unwrap_or(0)
}
