// Putting The Rust Standard Library here
use std::env;
use std::io;
use std::io::Write;
use std::fs;

mod chunk;
mod debug;
mod value;
mod vm;
mod compiler;
mod scanner;
mod token_type;
mod precedence;

use vm::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut vm: VM = VM::new();
    let args: Vec<_> = env::args().collect();
    // Returns error if too many arguments passed
    if args.len() > 2 {
        println!("Usage: clox [path]\n");
        std::process::exit(64);
    }
    // Runs the file of the directory of the second command
    // do 'cargo run test.lox', for example
    else if args.len() == 2 {
        run_file(&args[1], &mut vm);
    }
    // If no arguments are passed, run REPL 
    else {
        repl(&mut vm);
    }
}

// Runs file if correct path is specified
fn run_file(path : &String, vm: &mut VM) {
    let mut source: String = fs::read_to_string(path).expect("ERROR: Could not read file. Check directory is right or that the file is in the root folder");
    source.push('\0');
    let result = vm.interpret(source);

    if result == InterpretResult::InterpretCompilerError {std::process::exit(65);}
    if result == InterpretResult::InterpretRuntimeError {std::process::exit(70);}
    
}

/*
BIG BUG: Repl doesn't seem to work well. When scanning, when looking for character it loops infinitely
Runs REPL in loop, executing line by line
*/  
fn repl(vm: &mut VM) {
    loop{
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).expect("Could not read the line");
        line.push('\0');
        vm.interpret(line);
        println!("\n");
    }
}
