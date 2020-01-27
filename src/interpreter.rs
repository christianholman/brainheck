use crate::compiler;
use crate::instruction::Instruction;
use crate::procedure::Procedure;

pub struct Interpreter {
    memory: Vec<u8>,
    pointer: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            memory: vec![0; 30000],
            pointer: 0,
        }
    }

    pub fn run(&mut self, software: &str) {
        let mut procedure = compiler::compile(software);
        procedure.push(Instruction::EOF);
        self.run_procedure(&procedure)
    }

    fn run_procedure(&mut self, procedure: &Procedure) {
        // Run through every instruction in the incoming procedure
        for instruction in procedure {
            match instruction {
                Instruction::MoveRight(n) => {
                    self.pointer += n;
                }
                Instruction::MoveLeft(n) => {
                    self.pointer -= n;
                }
                Instruction::Increment(n) => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(*n);
                }
                Instruction::Decrement(n) => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(*n);
                }
                Instruction::Output(n) => {
                    for _ in 0..*n {
                        print!("{}", self.memory[self.pointer] as char);
                    }
                }
                Instruction::Loop(inner_procedure) => {
                    if self.memory[self.pointer] != 0 {
                        self.run_procedure(&inner_procedure);
                    }
                }
                Instruction::EOF => {
                    return;
                }
                // If the instruction isn't in our instruction set, do nothing, regard it as a
                // comment
                _ => (),
            }
        }

        if self.memory[self.pointer] != 0 {
            self.run_procedure(procedure);
        }
    }
}
