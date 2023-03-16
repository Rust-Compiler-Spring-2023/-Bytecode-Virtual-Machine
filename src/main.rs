mod chunk;
mod debug;
mod value;

use chunk::*;
use debug::*;
use value::*;

fn main() {
    let mut chunk: Chunk  = Chunk::new();
    let constant : u8 = chunk.add_constant(1.2);

    chunk.write_chunk_opcode(OpCode::OpConstant, 123);
    chunk.write_chunk_u8(constant, 123);

    chunk.write_chunk_opcode(OpCode::OpReturn, 123);
    disassemble_chunk(&chunk, "test chunk");
    chunk.free_chunk();
}
