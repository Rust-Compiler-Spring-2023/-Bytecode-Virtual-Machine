use crate::chunk::*;

pub fn disassemble_chunk(chunk: &Chunk, name: String){
    println!("== {name} ==\n");

    let mut offset : usize = 0;
    while offset < chunk.code.len(){
        offset = disassemble_instruction(chunk, offset);
    }

}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize{
    // print the number of leading zeros
    print!("{offset:04}");

    let instruction : OpCode = chunk.code[offset].into();
    match instruction{
        OpCode::OpReturn => simple_instruction("OpReturn", offset),  
    }

    
}