use crate::chunk::*;
use crate::debug::*;
use crate::value::*;
use crate::compiler::*;
pub struct VM{
    // chunk : Chunk,
    ip : usize,
    stack : Vec<Value>,
    compiler : Compiler,
}

#[derive(Debug,PartialEq)]
pub enum InterpretResult{
    InterpretOk,
    InterpretCompilerError,
    InterpretRuntimeError
}

impl VM{
    pub fn new() -> Self{
        VM{
            ip : 0,
            stack : Vec::new(),
            compiler : Compiler::new(),
        }
    }

    pub fn free_vm(&mut self){
        self.stack = Vec::new();
        self.ip = 0;
    }

    // reads the byte currently pointed at by ip and then advances the instruction pointer
    fn read_byte(&mut self, chunk : &Chunk) -> OpCode{
        let curr_ip = self.ip;
        self.ip += 1;
        chunk.code[curr_ip].into()
    }

    // reads the next byte from the bytecode, treats the resulting number as an index, 
    // and looks up the corresponding Value in the chunk’s constant table.
    fn read_constant(&mut self, chunk : &Chunk) -> Value{
        let curr_byte: usize = self.read_byte(chunk) as usize;
        chunk.constants.values[curr_byte]
    }

    fn run(&mut self, chunk : &Chunk) -> InterpretResult{
        
       loop{
            
            #[cfg(feature = "debug_trace_execution")]
            self.debug(chunk);

            let instruction : OpCode = self.read_byte(chunk);
            match instruction{
                OpCode::OpConstant => {
                    let constant: Value = self.read_constant(chunk);
                    self.push(constant);
                    // break ?
                },
                OpCode::OpAdd => {
                    let b : Value = self.pop();
                    let a : Value = self.pop();
                    self.push(a + b);
                },
                OpCode::OpSubtract => {
                    let b : Value = self.pop();
                    let a : Value = self.pop();
                    self.push(a - b);
                },
                OpCode::OpMultiply => {
                    let b : Value = self.pop();
                    let a : Value = self.pop();
                    self.push(a * b);
                },
                OpCode::OpDivide => {
                    let b : Value = self.pop();
                    let a : Value = self.pop();
                    self.push(a / b);
                },
                OpCode::OpNegate => {
                    let negated_value = -self.pop();
                    self.push(negated_value)
                },
                OpCode::OpReturn => {
                    print_value(self.pop());
                    println!("");
                    return InterpretResult::InterpretOk
                }
            }
        }
    }

    fn debug(&mut self, chunk : &Chunk){
        print!("          ");
        let mut copy_stack: Vec<Value> = self.stack.clone();
        while !copy_stack.is_empty(){
            print!("[ ");
            print_value(copy_stack.pop().unwrap());
            print!(" ]");
        }
        println!("");
        disassemble_instruction(chunk, self.ip);
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult{
        let mut chunk = Chunk::new();

        // If the compiler encounters an error, compile() returns false and we discard the unusable chunk.
        if !self.compiler.compile(source, &chunk) {
            chunk.free_chunk();
            return InterpretResult::InterpretCompilerError;
        }

        let result = self.run(&chunk);
        chunk.free_chunk();
        return result;
    }

    pub fn push(&mut self, value: Value){
        self.stack.push(value)
    }

    pub fn pop(&mut self) -> Value{
        self.stack.pop().unwrap()
    }


}