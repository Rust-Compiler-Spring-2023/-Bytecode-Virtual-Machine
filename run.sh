#!/bin/bash
echo "Enter file name:"
read file_name
touch "../test/$file_name.lox"
echo "Created file: ../test/$file_name.lox"
vim -c "startinsert" ../test/$file_name.lox
cargo run "../test/$file_name.lox"
