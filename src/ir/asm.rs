use crate::bounded::Bounded;

use super::expr::Operator;

pub type Xmm = Bounded<0, 15>;

pub enum R {
    /// Does not need to be saved. Holds return value of program.
    Rax,
    /// Needs to be saved.
    Rbx,
    /// Does not need to be saved. Holds the 4th argument to the program.
    Rcx,
    /// Does not need to be saved. Holds the 3rd argument to the program.
    Rdx,
    /// Does not need to be saved. Holds the 2nd argument to the program.
    Rsi,
    /// Does not need to be saved. Holds the 1st argument to the program.
    Rdi,
    /// Needs to be saved.
    Rbp,
    /// Needs to be saved. Points to the top of the stack.
    Rsp,
    /// Does not need to be saved.
    RLow(Bounded<8, 11>),
    /// Needs to be saved.
    RHigh(Bounded<11, 15>),
}

/// Most x64 registers under linux calling convention
pub enum Register {
    /// Does not need to be saved.
    Xmm(Xmm),
    /// Sometimes needs to be saved.
    R(R),
}

pub enum ScaleFactor {
    S1,
    S2,
    S4,
    S8,
}

pub struct Memory {
    displacement: i32,
    base: R,
    index: R,
    scale: ScaleFactor,
}

pub enum Move {
    /// movss
    FloatFromMemory { dest: Xmm, src: Memory },
    /// movss
    FloatToMemory { dest: Memory, src: Xmm },
    /// movss
    FloatToFloat { dest: Xmm, src: Xmm },
}

// addss, subss, mulss, divss
pub struct FloatAssign {
    operator: Operator,
    dest: Xmm,
    value: Xmm,
}

pub enum Arithmetic {
    FloatAssign(FloatAssign),
}

pub enum Compare {
    /// comiss
    CompareFloats,
}

pub enum Jump {
    /// jmp
    /// Unconditionally jump `offset` *instructions*, not bytes
    Unconditional { offset: i32 },
    /// jae
    /// Conditionally jump `offset` *instructions*, not bytes
    AboveEqual { offset: i32 },
}

pub enum Instruction {
    Move(Move),
    ArithmeticOperation(Arithmetic),
    Compare(Compare),
    Jump(Jump),
}

pub struct Program {
    instructions: Vec<Instruction>,
}
