# Introduction
A brainfuck interpreter made to practice programming lol
It's a little bit optimized, combines trailing +'s and -'s as well as . 

## What is brainfuck?
Brainfuck is an esoteric programming language with only 8 instructions.

### Example program:
```brainfuck
+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.
```
### Output:
```
hello world
```
[Source](https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages#Brainfuck)
# Usage
```
cargo run <file> --release
```

--release for the big speed™
