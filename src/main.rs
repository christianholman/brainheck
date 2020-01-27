use bf::Interpreter;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut interp = Interpreter::new();
    interp.run(&std::fs::read_to_string(args[1].clone()).expect("Error while reading file"));
}
