use arrayvec::ArrayVec;

pub struct Bytes {
    bytes: Vec<u8>,
}

pub struct InstructionBuilder {
    legacy_prefix: Option<ArrayVec<u8, 4>>,
    opcode: ArrayVec<u8, 4>,
    mod_reg_rm: Option<u8>,
    sib: Option<u8>,
    displacement: Option<ArrayVec<u8, 8>>,
    immediate: Option<ArrayVec<u8, 8>>,
}

impl Bytes {
    fn push(&mut self, instruction: InstructionBuilder) -> &mut Self {
        self.bytes
            .extend(instruction.legacy_prefix.iter().flatten());
        self.bytes.extend(instruction.opcode);
        self.bytes.extend(instruction.mod_reg_rm);
        self.bytes.extend(instruction.sib);
        self.bytes.extend(instruction.displacement.iter().flatten());
        self.bytes.extend(instruction.immediate.iter().flatten());
        self
    }
}

fn to_arrayvec<const N: usize>(elements: impl IntoIterator<Item = u8>) -> ArrayVec<u8, N> {
    let mut array = ArrayVec::new();
    for byte in elements {
        array.push(byte);
    }
    array
}

impl InstructionBuilder {
    pub fn new(opcode: impl IntoIterator<Item = u8>) -> Self {
        Self {
            legacy_prefix: None,
            opcode: to_arrayvec(opcode),
            mod_reg_rm: None,
            sib: None,
            displacement: None,
            immediate: None,
        }
    }

    /// mod is 2 bits
    /// reg is 3 bits
    /// rm is 3 bits
    pub fn mod_reg_rm(&mut self, mod_: u8, reg: u8, rm: u8) -> &mut Self {
        assert!(mod_ <= 0b11);
        assert!(reg <= 0b111);
        assert!(rm <= 0b111);

        let byte = mod_ << 6 + reg << 3 + rm;

        self.mod_reg_rm = Some(byte);
        self
    }

    /// scale is 2 bytes
    /// index is 3 bytes
    /// base is 3 bytes
    pub fn sib(&mut self, scale: u8, index: u8, base: u8) -> &mut Self {
        assert!(scale <= 0b11);
        assert!(index <= 0b111);
        assert!(base <= 0b111);

        let byte = scale << 6 + index << 3 + base;

        self.sib = Some(byte);
        self
    }

    pub fn displacement(&mut self, displacement: impl IntoIterator<Item = u8>) -> &mut Self {
        self.displacement = Some(to_arrayvec(displacement));
        self
    }

    pub fn immediate(&mut self, immediate: impl IntoIterator<Item = u8>) -> &mut Self {
        self.immediate = Some(to_arrayvec(immediate));
        self
    }

    pub fn legacy_prefix(&mut self, legacy_prefix: impl IntoIterator<Item = u8>) -> &mut Self {
        self.legacy_prefix = Some(to_arrayvec(legacy_prefix));
        self
    }
}
