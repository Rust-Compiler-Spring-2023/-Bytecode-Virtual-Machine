use crate::value::*;

#[derive(Debug)]
pub enum OpCode {
    // Enum can hold a value
    // once it holds value, the rest will automatically have the incremented value of the previous
    OpConstant = 0,
    OpNil,
    OpTrue,
    OpFalse,
    OpPop,
    OpGetLocal,
    OpSetLocal,
    OpGetGlobal,
    OpDefineGlobal,
    OpSetGlobal,
    OpEqual,
    OpGreater,
    OpLess,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNot,
    OpNegate,
    OpPrint,
    OpJump,
    OpJumpIfFalse,
    OpLoop,
    OpReturn,
}

#[derive(Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>
}

impl Chunk {
    pub fn new() -> Self {
        Chunk{
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new()
        }
    }

    pub fn write_chunk_u8(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_chunk_opcode(&mut self, op_code: OpCode, line: usize) {
        self.code.push(op_code as u8);
        self.lines.push(line);
    }

    // Freeing the chunk will free the old vector and initialize a new one
    pub fn free_chunk(&mut self) {
        self.code = Vec::new();
        self.lines = Vec::new();
        self.constants = Vec::new();
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        // return casts usize to u8
        // We need this return in order to get the index of the constant we just added
        return (self.constants.len() - 1).try_into().unwrap();
    }
}

// Changes u8 to OpCode. Use .into() to change
impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::OpConstant,
            1 => OpCode::OpNil,
            2 => OpCode::OpTrue,
            3 => OpCode::OpFalse,
            4 => OpCode::OpPop,
            5 => OpCode::OpGetLocal,
            6 => OpCode::OpSetLocal,
            7 => OpCode::OpGetGlobal,
            8 => OpCode::OpDefineGlobal,
            9 => OpCode::OpSetGlobal,
            10 => OpCode::OpEqual,
            11 => OpCode::OpGreater,
            12 => OpCode::OpLess,
            13 =>OpCode::OpAdd,
            14 => OpCode::OpSubtract,
            15 => OpCode::OpMultiply,
            16 => OpCode::OpDivide,
            17 => OpCode::OpNot,
            18 => OpCode::OpNegate,
            19 => OpCode::OpPrint,
            20 => OpCode::OpJump,
            21 => OpCode::OpJumpIfFalse,
            22 => OpCode::OpLoop,
            23 => OpCode::OpReturn,
            ///////////////////////////////////
            //// Could create possible bug ////
            ///////////////////////////////////
            _ => {
                println!("Value not avaliable: {:?}", value);
                panic!()
            }
        }
    }
}
