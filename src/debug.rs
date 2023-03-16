use crate::chunk::*;
use crate::value::*;

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
        OpCode::OpConstant => constant_instruction("OpConstant", chunk, offset),
        OpCode::OpReturn => simple_instruction("OpReturn", offset),
        _ => {
            println!("Unknown opcode {:#?}", instruction);
            offset + 1
        }
    }    
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize{
    let constant: u8 = chunk.code[offset + 1];
    print!("{name:-16} {constant:4} '");
    print_value(chunk.constants.values[constant as usize]);
    println!("'");
    return offset + 2;

}

fn simple_instruction(name: &str, offset: usize) -> usize{
    println!("{}", name);
    return offset + 1;
}