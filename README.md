# Bytecode-Virtual-Machine
A rust bytecode virtual machine implementation of the Lox language from [Crafting Interpreters](https://craftinginterpreters.com) by Robert Nystrom.

## How to build
In order to build the project you will need to download Rust's build system and package manager **cargo**. On linux and macOS systems, this is done as follows:
```bash
curl https://sh.rustup.rs -sSf | sh
```
If you're having trouble you can visit the following links for more information:
* [Install Rust and Cargo - The Cargo Book](https://doc.rust-lang.org/cargo/getting-started/installation.html)
* [Installation - The Rust Programming Language](https://doc.rust-lang.org/book/ch01-01-installation.html)


## How to run
There are two ways to run the program.
1) Run the REPL
2) Run a file
 
If you wish to run the REPL, simply write 
```bash
cargo run
```
If you wish to run a file, simply add the directory of the file relative to the root folder. For example, if you wish to run one of the test files write
```bash
cargo run test/test_tokens.lox
```


## How To Run in Debug Mode
You can also include the following flag in order to exectute in debug mode
```bash
cargo run --features debug_trace_execution
```
For now, this will show you every detail of the bytecode in detail.

For example, you might see something like this
```bash
0000  123 OP_CONSTANT         0 '1.2'
```
The first 4 digis (0000 in this instance) tell you the offset from the beginning of the
vector that holds the bytes of operations. The second number (123) tell you the line the 
instruction is on the user's program. If the instruction is in the same line as the previous one
a '|' will instead appear. Following that is the name of the operation. After that, all the way
to the right, the first number (0) is the index number for the value digit in the value array.
Lastly, it's the value itself.

If you wish to run a file with debug mode, simply add the file directory after the flags. For example,
```bash
cargo run --features debug_trace_execution test/test_tokens.lox
```
There are test files in the test folder. Like shown previously, you can put the name of the file after the "test/"

## Bugs in the Interpreter
 - A constant variable can be overwritten by a var keyword. Example:
 ```bash
 const x = 10;
 var x = 20;
 print x; // Prints 20
 ```
 


