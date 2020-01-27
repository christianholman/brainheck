pub use instruction::Instruction;
pub use interpreter::Interpreter;

mod compiler;
mod errors;
mod instruction;
mod interpreter;
mod procedure;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
