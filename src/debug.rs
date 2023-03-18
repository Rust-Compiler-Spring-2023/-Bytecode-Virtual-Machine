use crate::chunk::*;
use crate::value::*;

pub fn disassemble_chunk(chunk: &Chunk, name: &str){
    println!("== {name} ==");

    let mut offset : usize = 0;
    while offset < chunk.code.len(){
        offset = disassemble_instruction(chunk, offset);
    }

}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize{
    // print the byte offset of instruction
    // Tells us where in the chunk this instruction is
    print!("{offset:04} ");

    // We show a | for any instruction that comes from the same source line as the preceding one
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1]{
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }

    let instruction : OpCode = chunk.code[offset].into();
    match instruction{
        OpCode::OpConstant => constant_instruction("OpConstant", chunk, offset),
        OpCode::OpAdd => simple_instruction("OpAdd", offset),
        OpCode::OpSubtract => simple_instruction("OpSubtract", offset),
        OpCode::OpMultiply => simple_instruction("OpMultiply", offset),
        OpCode::OpDivide => simple_instruction("OpDivide", offset),
        OpCode::OpNegate => simple_instruction("OpNegate", offset),
        OpCode::OpReturn => simple_instruction("OpReturn", offset),
        _ => {
            println!("Unknown opcode {:#?}", instruction);
            offset + 1
        }
    }    
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize{
    // constant is the index of the 
    let constant_index: u8 = chunk.code[offset + 1];
    print!("{name:-16} {constant_index:4} '");
    print_value(chunk.constants.values[constant_index as usize]);
    println!("'");
    return offset + 2;

}

fn simple_instruction(name: &str, offset: usize) -> usize{
    println!("{}", name);
    return offset + 1;
}