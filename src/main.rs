mod parser;
mod evaluator;
mod drawing;

fn main() {
    let parse = parser::program::parse_program("to star repeat 5 [ fd 100 rt 144 ] end clearscreen star");
    let parsed_program = match parse {
        Ok((_, blocks)) => {blocks}
        Err(e) => {eprintln!("{}", e); Vec::new()}
    };
    let mut env = evaluator::environment::Env::new();
    let mut turtle = evaluator::turtle::Turtle::new(800.0, 600.0);
    evaluator::eval::eval(parsed_program, &mut turtle, &mut env);
    drawing::draw::save_image(turtle, "gwiazdka");
}

