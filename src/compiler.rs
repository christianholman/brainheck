use crate::errors::UnmatchedBracketsError;
use crate::instruction::Instruction;
use crate::procedure::Procedure;

use std::collections::HashMap;

pub fn compile(program: &str) -> Procedure {
    let mut procedure: Procedure = vec![];
    let pairs = match_brackets(program).unwrap();

    let mut i = 0;
    let mut trailing = ' ';
    let mut trailing_val = 0usize;

    while i < program.len() {
        let instr = program.chars().nth(i).expect("No instructions in program");

        if trailing != instr {
            match trailing {
                '>' => procedure.push(Instruction::MoveRight(trailing_val)),
                '<' => procedure.push(Instruction::MoveLeft(trailing_val)),
                '+' => procedure.push(Instruction::Increment(trailing_val as u8)),
                '-' => procedure.push(Instruction::Decrement(trailing_val as u8)),
                '.' => procedure.push(Instruction::Output(trailing_val)),
                _ => (),
            }
            trailing = instr;
            trailing_val = 0;
        }

        match instr {
            '>' | '<' | '+' | '-' | '.' => trailing_val += 1,
            '[' => {
                procedure.push(Instruction::Loop(compile(
                    &program[i + 1..pairs[&i]].chars().collect::<String>(),
                )));
                i = pairs[&i];
            }
            _ => (),
        }
        i += 1;
    }

    match trailing {
        '>' => procedure.push(Instruction::MoveRight(trailing_val)),
        '<' => procedure.push(Instruction::MoveLeft(trailing_val)),
        '+' => procedure.push(Instruction::Increment(trailing_val as u8)),
        '-' => procedure.push(Instruction::Decrement(trailing_val as u8)),
        '.' => procedure.push(Instruction::Output(trailing_val)),
        _ => (),
    }

    procedure
}

fn match_brackets(program: &str) -> Result<HashMap<usize, usize>, UnmatchedBracketsError> {
    let mut stack: Vec<usize> = vec![];
    let mut stack_map: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;

    while i < program.len() {
        let instruction = program.chars().nth(i).expect("No instructions in program");
        match instruction {
            '[' => {
                stack.push(i);
            }
            ']' => {
                stack_map.insert(
                    match stack.pop() {
                        Some(opening_bracket) => opening_bracket,
                        None => return Err(UnmatchedBracketsError),
                    },
                    i,
                );
            }
            _ => (),
        }
        i += 1;
    }

    if stack.len() != 0 {
        return Err(UnmatchedBracketsError);
    }

    Ok(stack_map)
}
