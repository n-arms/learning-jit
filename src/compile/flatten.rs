use crate::ir::{expr, register};

struct RegisterSource {
    unused: usize,
}

impl RegisterSource {
    fn new(unused: usize) -> Self {
        Self { unused }
    }

    fn fresh(&mut self) -> register::Register {
        let index = self.unused;
        self.unused += 1;
        register::Register { index }
    }
}

#[derive(Default)]
struct ProgramBuilder {
    statements: Vec<register::Statement>,
}

impl ProgramBuilder {
    fn with_statement(&mut self, statement: register::Statement) {
        self.statements.push(statement);
    }
}

fn flatten(
    expr: &expr::Expr,
    registers: &mut RegisterSource,
    program: &mut ProgramBuilder,
) -> register::Value {
    match expr {
        expr::Expr::Operation { operator, operands } => {
            let a = flatten(&operands[0], registers, program);
            let b = flatten(&operands[1], registers, program);

            let result = registers.fresh();

            program.with_statement(register::Statement {
                destination: result,
                expr: register::Expr::Move(a),
            });

            program.with_statement(register::Statement {
                destination: result,
                expr: register::Expr::Operation {
                    operator: *operator,
                    operand: b,
                },
            });

            register::Value::Register(result)
        }
        expr::Expr::Variable(index) => {
            register::Value::Register(register::Register { index: *index })
        }
        expr::Expr::Number(number) => register::Value::Number(*number),
        expr::Expr::IfPositive(if_positive) => {
            let predicate = flatten(&if_positive.predicate, registers, program);
            let consequent = flatten(&if_positive.consequent, registers, program);
            let alternative = flatten(&if_positive.alternative, registers, program);

            let result = registers.fresh();

            program.with_statement(register::Statement {
                destination: result,
                expr: register::Expr::IfPositive {
                    predicate,
                    consequent,
                    alternative,
                },
            });

            register::Value::Register(result)
        }
    }
}

pub fn to_program(expr: &expr::Expr, input: Vec<register::Register>) -> register::Program {
    let mut registers =
        RegisterSource::new(input.iter().map(|reg| reg.index + 1).max().unwrap_or(0));
    let mut program = ProgramBuilder::default();
    let output = flatten(expr, &mut registers, &mut program);

    register::Program {
        input,
        statements: program.statements,
        output,
    }
}
