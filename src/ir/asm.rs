use std::fmt;

use crate::bounded::Bounded;

use super::expr::Operator;

pub type Xmm = Bounded<0, 15>;

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum ScaleFactor {
    S1,
    S2,
    S4,
    S8,
}

pub struct Index {
    pub index: R,
    pub scale: ScaleFactor,
}

pub struct Memory {
    pub displacement: i32,
    pub base: R,
    pub index: Option<Index>,
}

pub enum Move {
    /// movss
    FloatFromMemory {
        dest: Xmm,
        src: Memory,
    },
    /// movss
    FloatToMemory {
        dest: Memory,
        src: Xmm,
    },
    /// movss
    FloatToFloat {
        dest: Xmm,
        src: Xmm,
    },
    IntegerFromConstant {
        dest: R,
        src: i32,
    },
    FloatFromInteger {
        dest: Xmm,
        src: R,
    },
    IntegerToMemory {
        dest: Memory,
        src: R,
    },
}

// addss, subss, mulss, divss
pub struct FloatAssign {
    pub operator: Operator,
    pub dest: Xmm,
    pub value: Xmm,
}

pub struct IntegerAssign {
    pub dest: R,
    pub value: u32,
}

pub enum Arithmetic {
    FloatAssign(FloatAssign),
    IntegerAddAssign(IntegerAssign),
    IntegerSubAssign(IntegerAssign),
}

pub enum Compare {
    /// comiss
    CompareFloats { first: Xmm, second: Xmm },
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

#[derive(Debug)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Move(mov) => mov.fmt(f),
            Instruction::ArithmeticOperation(arithmetic) => arithmetic.fmt(f),
            Instruction::Compare(compare) => compare.fmt(f),
            Instruction::Jump(jump) => jump.fmt(f),
        }
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::FloatFromMemory { dest, src } => {
                write!(f, "%xmm{:?} = {:?}", Into::<i64>::into(*dest), src)
            }
            Move::FloatToMemory { dest, src } => {
                write!(f, "{:?} = %xmm{:?}", dest, Into::<i64>::into(*src))
            }
            Move::FloatToFloat { dest, src } => {
                write!(
                    f,
                    "%xmm{:?} = %xmm{:?}",
                    Into::<i64>::into(*dest),
                    Into::<i64>::into(*src)
                )
            }
            Move::IntegerFromConstant { dest, src } => write!(f, "{:?} = {}", dest, src),
            Move::FloatFromInteger { dest, src } => {
                write!(f, "%xmm{:?} = {:?}", Into::<i64>::into(*dest), src)
            }
            Move::IntegerToMemory { dest, src } => write!(f, "{:?} = {:?}", dest, src),
        }
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.displacement != 0 {
            write!(f, "{}", self.displacement)?;
        }
        write!(f, "[{:?}", self.base)?;
        if let Some(Index { index, scale }) = &self.index {
            write!(f, " + {:?} * {:?}", index, scale)?;
        }
        write!(f, "]")
    }
}

impl fmt::Debug for R {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            R::Rax => "rax",
            R::Rbx => "rbx",
            R::Rcx => "rcx",
            R::Rdx => "rdx",
            R::Rsi => "rsi",
            R::Rdi => "rdi",
            R::Rbp => "rbp",
            R::Rsp => "rsp",
            R::RLow(reg) => {
                let index: i64 = (*reg).into();
                return write!(f, "%r{}", index);
            }
            R::RHigh(reg) => {
                let index: i64 = (*reg).into();
                return write!(f, "%r{}", index);
            }
        };
        write!(f, "%{}", str)
    }
}

impl Into<u8> for ScaleFactor {
    fn into(self) -> u8 {
        match self {
            ScaleFactor::S1 => 1,
            ScaleFactor::S2 => 2,
            ScaleFactor::S4 => 4,
            ScaleFactor::S8 => 8,
        }
    }
}

impl fmt::Debug for ScaleFactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num: u8 = (*self).into();
        write!(f, "{}", num)
    }
}

impl fmt::Debug for Arithmetic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Arithmetic::FloatAssign(float) => float.fmt(f),
            Arithmetic::IntegerAddAssign(IntegerAssign { dest, value }) => {
                write!(f, "{:?} += {}", dest, value)
            }
            Arithmetic::IntegerSubAssign(IntegerAssign { dest, value }) => {
                write!(f, "{:?} -= {}", dest, value)
            }
        }
    }
}

impl fmt::Debug for FloatAssign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?}= {:?}", self.dest, self.operator, self.value)
    }
}

impl fmt::Debug for Compare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Compare::CompareFloats { first, second } => write!(
                f,
                "{:?} vs {:?}",
                Into::<i64>::into(*first),
                Into::<i64>::into(*second)
            ),
        }
    }
}

impl fmt::Debug for Jump {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Jump::Unconditional { offset } => write!(f, "jump {}", offset),
            Jump::AboveEqual { offset } => write!(f, "if above, jump {}", offset),
        }
    }
}
