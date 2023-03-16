use crate::value::*;

#[derive(Debug)]
pub enum OpCode{
    OpConstant = 0,
    OpReturn
}

pub struct Chunk{
    pub code: Vec<u8>,
    pub constants : ValueArray
}

impl Chunk{
    pub fn new() -> Self{
        Chunk{
            code : Vec::new(),
            constants : ValueArray::new()
        }
    }

    pub fn write_chunk_u8(&mut self, byte: u8){
        self.code.push(byte);
    }

    pub fn write_chunk_opcode(&mut self, op_code: OpCode){
        self.code.push(op_code.into());
    }

    // Freeing the chunk will free the old vector and initialize a new one
    pub fn free_chunk(&mut self){
        self.code = Vec::new();
        self.constants= ValueArray::new();
    }

    pub fn add_constant(&mut self, value: Value) -> u8{
        self.constants.write_value_array(value);
        // return casts usize to u8
        // We need this return in order to get the index of the constant we just added
        return (self.constants.values.len() - 1).try_into().unwrap();
    }
}



// Changes u8 to OpCode. Use .into() to change
impl From<u8> for OpCode{
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::OpConstant,
            1 => OpCode::OpReturn,
            _ => unimplemented!("")
        }
    }
}

// Changes OpCode to u8. Use .into() to change
impl From<OpCode> for u8{
    fn from(op_code: OpCode) -> Self {
        op_code as u8
    }
}



