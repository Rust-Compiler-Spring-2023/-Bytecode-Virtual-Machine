
#[derive(Debug)]
pub enum OpCode{
    OpReturn = 0,
}

pub struct Chunk{
    pub code: Vec<u8>
}

impl Chunk{
    pub fn new() -> Self{
        Chunk{
            code : Vec::new()
        }
    }

    pub fn write_chunk_opcode(&mut self, byte: OpCode){
        self.code.push(byte.into());
    }

    // Freeing the chunk will free the old vector and initialize a new one
    pub fn free_chunk(&mut self){
        self.code = Vec::new();
    }

}



// Changes u8 to OpCode. Use .into() to change
impl From<u8> for OpCode{
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::OpReturn,
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



