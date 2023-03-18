# Bytecode-Virtual-Machine
A rust bytecode virtual machine implementation of the Lox language from [Crafting Interpreters](https://craftinginterpreters.com) by Robert Nystrom.

## How to run
In order to run the program, you can simply do 
```bash
cargo run
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
instruction is on the user's program. If the instruction is in the same lin as the previous one
a '|' will instead appear. Following that is the name of the operation. After that, all the way
to the right, the first number (0) is the index number for the value digit in the value array.
Lastly, it's the value itself.

As the project grows there will be more information.
