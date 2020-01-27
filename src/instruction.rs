use crate::procedure::Procedure;

#[derive(Debug)]
pub enum Instruction {
    // Move(n) left or right. Negative number moves left on the tape,
    // while positive number moves right
    MoveRight(usize),
    MoveLeft(usize),
    //  Change the value at pointer memory location
    Increment(u8),
    Decrement(u8),
    // Every loop is it's own procedure, just like the parent program.
    Loop(Procedure),
    // Output the current data at the pointer memory location as a char
    Output(usize),
    // Read STDIN as a char and write it to the memory at the pointer location.
    EOF,
}
