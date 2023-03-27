use crate::value::*;

#[derive(Debug)]
pub enum OpCode {
    // Enum can hold a value
    // once it holds value, the rest will automatically have the incremented value of the previous
    OpConstant = 0,
    OpNil,
    OpTrue,
    OpFalse,
    OpEqual,
    OpGreater,
    OpLess,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNot,
    OpNegate,
    OpReturn,
}

#[derive(Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
    pub lines: Vec<usize>
}

impl Chunk {
    pub fn new() -> Self {
        Chunk{
            code: Vec::new(),
            constants: ValueArray::new(),
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
        self.constants.free_value_array();
        self.constants = ValueArray::new();
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.write_value_array(value);
        // return casts usize to u8
        // We need this return in order to get the index of the constant we just added
        return (self.constants.values.len() - 1).try_into().unwrap();
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
            4 => OpCode::OpEqual,
            5 => OpCode::OpGreater,
            6 => OpCode::OpLess,
            7 => OpCode::OpAdd,
            8 => OpCode::OpSubtract,
            9 => OpCode::OpMultiply,
            10 => OpCode::OpDivide,
            11 => OpCode::OpNot,
            12 => OpCode::OpNegate,
            13 => OpCode::OpReturn,
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
