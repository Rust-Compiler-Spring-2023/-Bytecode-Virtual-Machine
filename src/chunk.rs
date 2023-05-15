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
    OpSetConstLocal,
    OpGetGlobal,
    OpDefineGlobal,
    OpSetGlobal,
    OpDefineConstGlobal,
    OpEqual,
    OpGreater,
    OpLess,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpExponent,
    OpModulus,
    OpNot,
    OpNegate,
    OpPrint,
    OpJump,
    OpJumpIfFalse,
    OpLoop,
    OpCall,
    OpReturn,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new()
        }
    }
    
    pub fn write_chunk(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    // Freeing the chunk will free the old vector and initialize a new one
    pub fn free_chunk(&mut self) {
        self.code = Vec::new();
        self.lines = Vec::new();
        self.constants = Vec::new();
    }

    // Adds to constant array and returns the index
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
            7 => OpCode::OpSetConstLocal,
            8 => OpCode::OpGetGlobal,
            9 => OpCode::OpDefineGlobal,
            10 => OpCode::OpSetGlobal,
            11 => OpCode::OpDefineConstGlobal,
            12 => OpCode::OpEqual,
            13 => OpCode::OpGreater,
            14 => OpCode::OpLess,
            15 => OpCode::OpAdd,
            16 => OpCode::OpSubtract,
            17 => OpCode::OpMultiply,
            18 => OpCode::OpDivide,
            19 => OpCode::OpExponent,
            20 => OpCode::OpModulus,
            21 => OpCode::OpNot,
            22 => OpCode::OpNegate,
            23 => OpCode::OpPrint,
            24 => OpCode::OpJump,
            25 => OpCode::OpJumpIfFalse,
            26 => OpCode::OpLoop,
            27 => OpCode::OpCall,
            28 => OpCode::OpReturn,
            ///////////////////////////////////
            //// Could create possible bug ////
            ///////////////////////////////////
            _ => {
                println!("Value not available: {:?}", value);
                panic!()
            }
        }
    }
}
