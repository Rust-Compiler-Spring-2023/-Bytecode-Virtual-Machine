mod chunk;
mod debug;

use chunk::*;
use debug::*;

fn main() {
    let mut chunk: Chunk  = Chunk::new();
    chunk.write_chunk_opcode(OpCode::OpReturn);
    disassemble_chunk(&chunk, "test chunk");
    chunk.free_chunk();
}
