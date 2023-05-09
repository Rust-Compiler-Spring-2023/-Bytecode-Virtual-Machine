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
The repl function, this takes input from the user in the terminal,
line by line, and executes the compiler with it. It uses a string that
stores the user's lines, one by one, continuously, and with that it can 
execute anything the compiler can, directly in the terminal.
*/
fn repl(_vm: &mut VM) {
    let mut line: String = String::new();
    loop{
        let mut vm: VM = VM::new();
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).expect("Could not read the line");
        line.push('\0');
        vm.interpret(line.clone());
        line.truncate(line.len()-1);
    }
}
