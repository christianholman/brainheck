use bf::Interpreter;

fn main() {
    let mut interp = Interpreter::new();
    interp.run(&std::fs::read_to_string("test").unwrap());
}
