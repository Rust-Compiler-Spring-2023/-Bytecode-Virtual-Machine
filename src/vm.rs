use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::SystemTime;
use std::borrow::Borrow;

use crate::chunk::*;
use crate::value::*;
use crate::compiler::*;
use crate::debug::*;


pub struct VM {
    frames: Vec<CallFrame>,
    stack : Vec<Value>,
    compiler : Compiler,
    globals : HashMap<String, Value>,
}

pub struct NativeClock{}

// Implements NativeFn trait for NativeClock
impl NativeFn for NativeClock{
    // CITE: UncleScientist lox-bytecode repo in Github
    // CITE: https://github.com/UncleScientist/lox-bytecode
    fn fun_call(&self, _arg_count: usize, _args: &[Value]) -> Value {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
            Ok(time) => Value::Number(time.as_millis() as f64),
            Err(_) => panic!("Can't get system time")
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompilerError,
    InterpretRuntimeError
}

pub struct CallFrame{
    function: Function,
    ip: RefCell<usize>,
    slots: usize
} 

// CITE: Learned to modify a RefCell object by a method by UncleScientist lox-bytecode repo in Github
// CITE: https://github.com/UncleScientist/lox-bytecode
impl CallFrame{
    fn increment_ip(&self, offset: usize){
        *self.ip.borrow_mut() += offset;
    }

    fn decrement_ip(&self, offset: usize){
        *self.ip.borrow_mut() -= offset;
    }
}

impl VM {
    pub fn new() -> Self {
        let mut vm = VM {
            frames: Vec::new(),
            stack : Vec::new(),
            compiler : Compiler::new(),
            globals : HashMap::new(),
        };
        let native_fun : Rc<dyn NativeFn> = Rc::new(NativeClock{});
        vm.define_native("clock".to_string(), &native_fun);
        vm
    }


    /**
     * reads the byte currently pointed at by ip and then advances the instruction pointer
     */
    fn read_byte(&mut self) -> OpCode {
        let ip = self.curr_frame().ip.clone();
        let ip = ip.into_inner();
        let curr_ip = ip;
        self.curr_frame().increment_ip(1);

        self.curr_frame().function.chunk.code[curr_ip].into()
    }

    /**
     * reads the next byte from the bytecode, treats the resulting number as an index,
     * and looks up the corresponding Value in the chunk’s constant table.
     */
    fn read_constant(&mut self) -> Value {
        let curr_byte: u8 = self.read_byte_u8();
        //println!("vm.rs:read_constant(): {:?}", chunk.constants);
        self.curr_frame().function.chunk.constants[curr_byte as usize].clone()
    }

    fn read_byte_u8(&mut self) -> u8 {
        let ip = self.curr_frame().ip.clone();
        let ip = ip.into_inner();
        let curr_ip = ip;
        self.curr_frame().increment_ip(1);
        //println!("vm.rs:read_byte_u8: {:?}", chunk.code);
        self.curr_frame().function.chunk.code[curr_ip]
    } 

    /**
     * Yanks the next two bytes from the chunk and builds a 16-bit unsigned integer out of them.
     */
    fn read_short(&mut self) -> usize {
        self.curr_frame().increment_ip(2);
        let ip = *self.curr_frame().ip.borrow();
        ((self.curr_frame().function.chunk.code[ip-2] as usize) << 8) | self.curr_frame().function.chunk.code[ip - 1] as usize
        
    }

    /**
     * Checks the type of operation
     * Using the top two values on the stack, it does the operation on them and pushes result back to stack
     */
    pub fn binary_op(&mut self, op: OpCode) -> InterpretResult{
            if !is_number(self.peek(0)) || !is_number(self.peek(1)){
                self.runtime_error("Operands must be numbers.");
                return InterpretResult::InterpretRuntimeError;
            }

            let b : Number = self.pop().into();
            let a : Number = self.pop().into();
            
            match op{
                OpCode::OpAdd => self.push(Value::from(a + b)),
                OpCode::OpSubtract => self.push(Value::from(a - b)),
                OpCode::OpMultiply => self.push(Value::from(a * b)),
                OpCode::OpDivide => self.push(Value::from(a / b)),
                OpCode::OpGreater => self.push(Value::from(a > b)),
                OpCode::OpLess => self.push(Value::from(a < b)),
                _ => ()
            }
        return InterpretResult::InterpretOk;
    }

    /**
     * Prints a stack trace
     */
    pub fn runtime_error(&mut self, error_message: &str){
        eprintln!("{}", error_message);
        
        for frame in self.frames.iter().rev() {
            let instruction = *frame.ip.borrow() - 1;
            let line =  frame.function.chunk.lines[instruction];
            let function_name = match frame.function.name.clone(){
                Some(name) => name,
                None => "script".to_string()
            };
            eprintln!("[line {line}] in {}", function_name);
        }
        
        self.stack.clear();
    }

    /**
     * Returns a reference to the current frame
     * CITE: UncleScientist lox-bytecode repo in Github
     * CITE: https://github.com/UncleScientist/lox-bytecode
     */
    fn curr_frame(&self) -> &CallFrame{
        self.frames.last().unwrap()
    }

    /**
     * Runs the bytecode given by the compiler
     */
    fn run(&mut self) -> InterpretResult {

        // let mut frame= self.curr_frame();

        loop {
            #[cfg(feature = "debug_trace_execution")]
            {
                print!("          ");
                let mut copy_stack: Vec<Value> = self.stack.clone();
                for item in copy_stack{
                    print!("[ {} ]", item.borrow());
                }
                println!("");
                // Debug ?
                // println!("run():offset: {}", self.get_ip());
                disassemble_instruction(&self.curr_frame().function.chunk, *self.curr_frame().ip.borrow());
            }

            let instruction: OpCode = self.read_byte();
            match instruction {
                OpCode::OpConstant => {
                    let constant: Value = self.read_constant();
                    self.push(constant);
                },
                OpCode::OpNil => {
                    self.push(Value::Nil);
                },
                OpCode::OpTrue => {
                    self.push(Value::from(true));
                },
                OpCode::OpFalse => {
                    self.push(Value::from(false));
                },
                OpCode::OpPop => {
                    self.pop();
                },
                OpCode::OpGetLocal => {
                    let slot = self.read_byte() as usize;
                    let slot_offset = self.curr_frame().slots;
                    self.push(self.stack[slot_offset + slot].clone());
                },
                OpCode::OpSetLocal => {
                    let slot = self.read_byte() as usize;
                    let slot_offset = self.curr_frame().slots;
                    self.stack[slot_offset + slot] = self.peek(0);
                },
                OpCode::OpGetGlobal => {
                    let name: String = self.read_constant().to_string();
                    let value: Value;
                    match self.globals.get(&name) {
                        Some(val) => { 
                            value = val.clone();
                            ()
                        },
                        None => {
                            println!("Undefined variable {}.", name);
                            return InterpretResult::InterpretRuntimeError;
                        }
                    }
                    self.push(value);
                },
                OpCode::OpDefineGlobal => { // 21.2
                    let name = self.read_constant().to_string();
                    let peeked_value = self.peek(0).clone(); 
                    self.globals.insert(name, peeked_value); 
                    self.pop();
                },
                OpCode::OpSetGlobal => {
                    let name: String = self.read_constant().to_string();
                    match self.globals.get(&name) {
                        Some(_val) => {
                            let insert_value = self.peek(0);
                            self.globals.insert(name, insert_value).unwrap();
                            ()
                        },
                        None => {
                            println!("Undefined variable {}", name);
                            return InterpretResult::InterpretRuntimeError;
                        }
                    }
                },
                OpCode::OpEqual => {
                    let b : Value = self.pop();
                    let a : Value = self.pop();
                    self.push(Value::from(b == a));
                }
                OpCode::OpGreater => {
                    self.binary_op(OpCode::OpGreater);
                },
                OpCode::OpLess => {
                    self.binary_op(OpCode::OpLess);
                },
                OpCode::OpAdd => {
                    if is_string(self.peek(0)) && is_string(self.peek(1)) {
                        self.concatenate();
                    }
                    else if is_number(self.peek(0)) && is_number(self.peek(1)) {
                        let b : Number = self.pop().into();
                        let a : Number = self.pop().into();
                        self.push(Value::from(a+b))
                    }
                    else {
                        self.runtime_error("Operands must be two numbers or two strings.");
                        return InterpretResult::InterpretRuntimeError
                    }
                },
                OpCode::OpSubtract => {
                    self.binary_op(OpCode::OpSubtract, );
                },
                OpCode::OpMultiply => {
                    self.binary_op(OpCode::OpMultiply, );
                },
                OpCode::OpDivide => {
                    self.binary_op(OpCode::OpDivide, );
                },
                OpCode::OpNot => {
                    let _pop: Value = self.pop();
                    self.push(Value::from(_pop.is_falsey()));
                },
                OpCode::OpPrint => {
                    print!("{}",self.pop());
                    println!("");
                },
                OpCode::OpNegate => {
                    if let Value::Number(_num) = self.peek(0){
                        self.runtime_error("Operand must be a number.");
                        
                        return InterpretResult::InterpretRuntimeError;
                    }
                    // Pop should be a Value::Number(_)
                    let _pop = self.pop();
                    // gets pushed to the stack<Value> 
                    self.push(_pop);
                },
                OpCode::OpJump => {
                    let offset = self.read_short();
                    // It doesn't check a condition and always applies the offset
                    self.curr_frame().increment_ip(offset);
                },
                OpCode::OpJumpIfFalse => {
                    let offset : usize = self.read_short();
                    if self.peek(0).is_falsey() {
                        self.curr_frame().increment_ip(offset);
                    }
                },
                OpCode::OpLoop => {
                    let offset: usize = self.read_short();
                    self.curr_frame().decrement_ip(offset);
                },
                OpCode::OpCall => {
                    let arg_count = self.read_byte() as usize;
                    let callee = self.peek(arg_count);
                    if !self.call_value(callee, arg_count){
                        return InterpretResult::InterpretRuntimeError
                    }
                },
                OpCode::OpReturn => {
                    let result = self.pop();
                    let prev_frame = self.frames.pop().unwrap();
                    if self.frames.len() == 0 {
                        self.pop();
                        return InterpretResult::InterpretOk;
                    }

                    self.stack.truncate(prev_frame.slots - 1);
                    self.stack.push(result)
                }
            }
            
        }
    }

    // Entry function for the VM
    pub fn interpret(&mut self, source: String) -> InterpretResult {
        
        let function: Option<Function> = self.compiler.compile(source);
        if function == None {return InterpretResult::InterpretCompilerError;}

        let function: Function = function.unwrap();
        self.push(Value::Fun(function.clone()));
        self.call(function, 0);
        
        let result = self.run();  
         
        result    
    }

    /**
     * Pushes the value to the stack
     */
    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    /**
     * Pops the value from the stack and returns the value
     */
    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    /**
     * Returns the value distance away from top of stack
     */
    pub fn peek(&mut self, distance: usize) -> Value{
        let len = self.stack.len() - 1;

        if distance > len{
            return Value::Nil;
        }
        
        self.stack[len - distance].clone()
    }

    /**
     * Initializes the next CallFrame on the stack.
     */
    pub fn call(&mut self, function: Function, arg_count: usize) -> bool{
        if arg_count != function.arity {
            self.runtime_error(&format!("Expected {} arguments but got {}", function.arity, arg_count));
            return false;
        }

        if self.frames.len() == 64 {
            self.runtime_error("Stack overflow.");
            return false;
        }

        let fun_name = function.clone().name;
        if fun_name == None{
            self.frames.push( CallFrame {
                function: function,
                ip: RefCell::new(0), 
                slots: self.stack.len() - arg_count as usize - 1
            });
            return true;
        }

        self.frames.push( CallFrame {
            function: function,
            ip: RefCell::new(0), 
            slots: self.stack.len() - arg_count as usize 
        });
        return true;
        
    }

    /**
     * Executes the appropriate function type, given the callee and argument count
     */
    pub fn call_value(&mut self, callee: Value, arg_count: usize) -> bool{
        match callee{
            Value::Fun(_function) => return self.call(_function, arg_count),
            Value::Native(_native_fun) => {
                let stack_len = self.stack.len();
                let result = _native_fun.fun_call(arg_count, &self.stack[stack_len - arg_count..stack_len]);
                self.stack.truncate(stack_len - arg_count + 1);
                self.push(result);
                true
            }
            _ => {
                self.runtime_error("Call only call functions and classes.");
                false
            }
        }
    }

    /**
     * Concatenates two strings into one
     */
    pub fn concatenate(&mut self) {
        let b : String = self.pop().into();
        let mut a : String = self.pop().into();

        a.push_str(&b);
        self.push(Value::String(a));
    }

    /**
     * Inserts the native function into the global variables
     */
    fn define_native(&mut self, name: String, function: &Rc<dyn NativeFn>){
        self.globals.insert(name, Value::Native(Rc::clone(function)));
    }
}

