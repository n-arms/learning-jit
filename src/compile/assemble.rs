use crate::ir::asm::{Arithmetic, Compare, Instruction, Jump, Move, Program};
use crate::ir::bytes::{Bytes, InstructionBuilder};

pub trait Assemblable {
    fn assemble(&self, bytes: &mut Bytes);
}

impl Assemblable for Program {
    fn assemble(&self, bytes: &mut Bytes) {
        for instruction in &self.instructions {
            instruction.assemble(bytes);
        }
    }
}

impl Assemblable for Instruction {
    fn assemble(&self, bytes: &mut Bytes) {
        match self {
            Instruction::Move(mov) => mov.assemble(bytes),
            Instruction::ArithmeticOperation(arithmetic) => arithmetic.assemble(bytes),
            Instruction::Compare(compare) => compare.assemble(bytes),
            Instruction::Jump(jump) => jump.assemble(bytes),
        }
    }
}

impl Assemblable for Move {
    fn assemble(&self, bytes: &mut Bytes) {
        todo!()
    }
}

impl Assemblable for Arithmetic {
    fn assemble(&self, bytes: &mut Bytes) {
        todo!()
    }
}

impl Assemblable for Compare {
    fn assemble(&self, bytes: &mut Bytes) {
        todo!()
    }
}

impl Assemblable for Jump {
    fn assemble(&self, bytes: &mut Bytes) {
        todo!()
    }
}
