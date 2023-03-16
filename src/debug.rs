use crate::chunk::*;

pub fn disassemble_chunk(chunk: &Chunk, name: &str){
    println!("== {name} ==");

    let mut offset : usize = 0;
    while offset < chunk.code.len(){
        offset = disassemble_instruction(chunk, offset);
    }

}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize{
    // print the byte offset of instruction
    // Tells us where in the chunk this instruction is
    print!("{offset:04} ");

    let instruction : OpCode = chunk.code[offset].into();
    match instruction{
        OpCode::OpReturn => simple_instruction("OpReturn", offset),
        _ => {
            println!("Unknown opcode {:#?}", instruction);
            offset + 1
        }
    }    
}

fn simple_instruction(name: &str, offset: usize) -> usize{
    println!("{}", name);
    return offset + 1;
}