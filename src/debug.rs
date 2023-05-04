use crate::chunk::*;
#[cfg(any(feature = "debug_trace_execution", feature = "debug_print_code"))]
pub fn disassemble_chunk(chunk: &Chunk, name: &str){
    println!("== {name} ==");

    let mut offset : usize = 0;
    while offset < chunk.code.len(){
        offset = disassemble_instruction(chunk, offset);
    }

}

#[cfg(any(feature = "debug_trace_execution", feature = "debug_print_code"))]
pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize{
    // print the byte offset of instruction
    // Tells us where in the chunk this instruction is

    use crate::chunk::OpCode;
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
        OpCode::OpNil => simple_instruction("OpNil", offset),
        OpCode::OpTrue => simple_instruction("OpTrue", offset),
        OpCode::OpFalse => simple_instruction("OpFalse", offset),
        OpCode::OpPop => simple_instruction("OpPop", offset),
        OpCode::OpGetLocal => byte_instruction("OpGetLocal", chunk, offset),
        OpCode::OpSetLocal => byte_instruction("OpSetLocal", chunk, offset),
        OpCode::OpGetGlobal => constant_instruction("OpGetGlobal", chunk,  offset),
        OpCode::OpDefineGlobal => constant_instruction("OpDefineGlobal", chunk, offset),
        OpCode::OpSetGlobal => constant_instruction("OpSetGlobal", chunk,  offset),
        OpCode::OpDefineConstGlobal => constant_instruction("OpDefineConstGlobal", chunk, offset),
        OpCode::OpEqual => simple_instruction("OpEqual", offset),
        OpCode::OpGreater => simple_instruction("OpGreater", offset),
        OpCode::OpLess => simple_instruction("OpLess", offset),
        OpCode::OpAdd => simple_instruction("OpAdd", offset),
        OpCode::OpSubtract => simple_instruction("OpSubtract", offset),
        OpCode::OpMultiply => simple_instruction("OpMultiply", offset),
        OpCode::OpDivide => simple_instruction("OpDivide", offset),
        OpCode::OpNot => simple_instruction("OpNot", offset),
        OpCode::OpNegate => simple_instruction("OpNegate", offset),
        OpCode::OpPrint => simple_instruction("OpPrint", offset),
        OpCode::OpJump => jump_instruction("OpJump", 1, chunk, offset),
        OpCode::OpJumpIfFalse => jump_instruction("OpJumpIfFalse", 1, chunk, offset),
        OpCode::OpLoop => jump_instruction("OpLoop", -1, chunk, offset),
        OpCode::OpCall => byte_instruction("OpCall", chunk, offset),
        OpCode::OpReturn => simple_instruction("OpReturn", offset),
        _ => {
            println!("Unknown opcode {:#?}", instruction);
            offset + 1
        }
    }    
}

#[cfg(any(feature = "debug_trace_execution", feature = "debug_print_code"))]
fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize{
    // constant is the index of the
    let constant_index: u8 = chunk.code[offset + 1];
    print!("{name:-16} {constant_index:4} '");
    print!("{}",chunk.constants[constant_index as usize]);
    println!("'");
    return offset + 2;

}

#[cfg(any(feature = "debug_trace_execution", feature = "debug_print_code"))]
fn simple_instruction(name: &str, offset: usize) -> usize{
    println!("{} ", name);
    return offset + 1;
}

#[cfg(any(feature = "debug_trace_execution", feature = "debug_print_code"))]
fn byte_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    // constant is the index of the 
    let slot: u8 = chunk.code[offset + 1];
    println!("{name:-16} {slot:4} ");
    return offset + 2;
}

#[cfg(any(feature = "debug_trace_execution", feature = "debug_print_code"))]
// Sign will tell us whether to jump back or forward (-1 or 1), so it can't be unsigned 
fn jump_instruction(name: &str, sign: i16, chunk: &Chunk, offset: usize) -> usize{
    let jump = (chunk.code[offset + 1] as usize) << 8 | chunk.code[offset + 2] as usize;
    let new_jump: usize;
    if sign == 1{
        new_jump = offset + 3 + jump;
    } else{
        new_jump = offset + 3 - jump;
    }
    println!("{name:-16} {offset:4} -> {new_jump}");
    offset + 3
}

