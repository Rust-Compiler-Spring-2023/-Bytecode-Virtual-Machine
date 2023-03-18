mod chunk;
mod debug;
mod value;
mod vm;

use chunk::*;
use debug::*;
use vm::*;

fn main() {
    let mut vm : VM = VM::new();
 
    let mut chunk: Chunk  = Chunk::new();
    
    //////// This will calculate (1.2 + 3.4) / 5.6

    let mut constant : u8 = chunk.add_constant(1.2);
    chunk.write_chunk_opcode(OpCode::OpConstant, 123);
    chunk.write_chunk_u8(constant, 123);

    constant = chunk.add_constant(3.4);
    chunk.write_chunk_opcode(OpCode::OpConstant, 123);
    chunk.write_chunk_u8(constant, 123);

    chunk.write_chunk_opcode(OpCode::OpAdd, 123);

    constant = chunk.add_constant(5.6);
    chunk.write_chunk_opcode(OpCode::OpConstant, 123);
    chunk.write_chunk_u8(constant, 123);

    chunk.write_chunk_opcode(OpCode::OpDivide, 123);
    chunk.write_chunk_opcode(OpCode::OpNegate, 123);

    chunk.write_chunk_opcode(OpCode::OpReturn, 123);
    //disassemble_chunk(&chunk, "test chunk");
    vm.interpret(&chunk);

    //vm.free_vm();
    chunk.free_chunk();
}

// This will be used for testing purposes
// In order to test code, create a function with the #[test] on top
// In the function test what you need and in the end of the function return assert_eq!(result, expected_result)
// Lastly, use cargo test to run test
// Create as many function tests as needed
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn testing_addition() {

    }
}

