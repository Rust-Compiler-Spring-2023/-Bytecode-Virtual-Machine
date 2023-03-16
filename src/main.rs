mod chunk;
mod debug;

use chunk::*;

fn main() {
    let mut chunk: Chunk  = Chunk::new();
    chunk.write_chunk_opcode(OpCode::OpReturn);

    chunk.free_chunk();
}
