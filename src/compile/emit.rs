use crate::ir::{asm, register};

fn register_access(register: register::Register) -> asm::Memory {
    let displacement = TryInto::<i32>::try_into(register.index).unwrap() * 4;

    asm::Memory {
        displacement,
        base: asm::R::Rsp,
        index: None,
    }
}

fn load_constant(number: f32, register: asm::R) -> asm::Instruction {
    asm::Instruction::Move(asm::Move::IntegerFromConstant {
        dest: register,
        src: i32::from_le_bytes(number.to_le_bytes()),
    })
}

fn load_value(
    value: register::Value,
    dest: asm::Xmm,
    intermediate: asm::R,
) -> Vec<asm::Instruction> {
    let mut instructions = Vec::new();
    match value {
        register::Value::Register(register) => {
            instructions.push(asm::Instruction::Move(asm::Move::FloatFromMemory {
                dest,
                src: register_access(register),
            }));
        }
        register::Value::Number(number) => {
            instructions.push(load_constant(number, intermediate));
            instructions.push(asm::Instruction::Move(asm::Move::FloatFromInteger {
                dest,
                src: intermediate,
            }));
        }
    }
    instructions
}

// all "registers" in `program` are actually references to memory.
pub fn emit_program(program: &register::Program, registers: u32) -> asm::Program {
    let mut instructions = Vec::new();

    let stack_allocation = registers * 4;

    instructions.push(asm::Instruction::ArithmeticOperation(
        asm::Arithmetic::IntegerSubAssign(asm::IntegerAssign {
            dest: asm::R::Rsp,
            value: stack_allocation,
        }),
    ));

    for statement in &program.statements {
        match statement.expr {
            register::Expr::Move(register::Value::Register(reg)) => {
                // loads `reg` into %xmm0, then loads %xmm0 into `statement.destination`

                let intermediate = 0.try_into().unwrap();
                instructions.push(asm::Instruction::Move(asm::Move::FloatFromMemory {
                    dest: intermediate,
                    src: register_access(reg),
                }));
                instructions.push(asm::Instruction::Move(asm::Move::FloatToMemory {
                    dest: register_access(statement.destination),
                    src: intermediate,
                }));
            }
            register::Expr::Move(register::Value::Number(number)) => {
                // loads `number` into %rbx, move %rbx into `statement.destination`
                let intermediate_int = asm::R::Rbx;
                instructions.push(load_constant(number, intermediate_int));
                instructions.push(asm::Instruction::Move(asm::Move::IntegerToMemory {
                    dest: register_access(statement.destination),
                    src: intermediate_int,
                }));
            }
            register::Expr::Operation { operator, operand } => {
                // loads `statement.destination` into %xmm0, `operand` into %xmm1, performs the operation, stores the result in `statement.destination`
                let first = 0.try_into().unwrap();
                let second = 1.try_into().unwrap();
                let intermediate = asm::R::Rbx;

                instructions.push(asm::Instruction::Move(asm::Move::FloatFromMemory {
                    dest: first,
                    src: register_access(statement.destination),
                }));

                load_value(operand, second, intermediate);

                instructions.push(asm::Instruction::ArithmeticOperation(
                    asm::Arithmetic::FloatAssign(asm::FloatAssign {
                        operator,
                        dest: first,
                        value: second,
                    }),
                ));

                instructions.push(asm::Instruction::Move(asm::Move::FloatToMemory {
                    dest: register_access(statement.destination),
                    src: first,
                }));
            }
            register::Expr::IfPositive {
                predicate,
                consequent,
                alternative,
            } => {
                // load `predicate`, `consequent` and `alternative` into %xmm0, %xmm1, %xmm2
                // load 0 into %xmm3
                // compare `predicate` to 0
                // conditionally skip an instruction
                // move alternative into consequent
                // store the result

                let predicate_xmm = 0.try_into().unwrap();
                let consequent_xmm = 1.try_into().unwrap();
                let alternative_xmm = 2.try_into().unwrap();
                let zero_xmm = 3.try_into().unwrap();
                let intermediate = asm::R::Rbx;

                load_value(predicate, predicate_xmm, intermediate);
                load_value(consequent, consequent_xmm, intermediate);
                load_value(alternative, alternative_xmm, intermediate);
                load_constant(0.0, intermediate);

                instructions.push(asm::Instruction::Move(asm::Move::FloatFromInteger {
                    dest: zero_xmm,
                    src: intermediate,
                }));

                instructions.push(asm::Instruction::Compare(asm::Compare::CompareFloats {
                    first: predicate_xmm,
                    second: zero_xmm,
                }));

                instructions.push(asm::Instruction::Jump(asm::Jump::AboveEqual { offset: 1 }));

                instructions.push(asm::Instruction::Move(asm::Move::FloatToFloat {
                    dest: consequent_xmm,
                    src: alternative_xmm,
                }));

                instructions.push(asm::Instruction::Move(asm::Move::FloatToMemory {
                    dest: register_access(statement.destination),
                    src: consequent_xmm,
                }));
            }
        }
    }

    instructions.push(asm::Instruction::ArithmeticOperation(
        asm::Arithmetic::IntegerAddAssign(asm::IntegerAssign {
            dest: asm::R::Rsp,
            value: stack_allocation,
        }),
    ));

    asm::Program { instructions }
}
