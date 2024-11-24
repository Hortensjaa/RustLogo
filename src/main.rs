mod parser;
mod evaluator;
mod drawing;
use std::fs;

// assuming input file name and output file name are the same
fn create_image(file_name: &str, size_x: f64, size_y: f64) {

    // read from file
    let filename = format!("codes/{}.txt", file_name);
    let program_code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_name, e);
            return;
        }
    };
    program_code.replace("\r\n\t", " ").trim().to_string();
    println!("File content: {}", program_code); // debug

    // parse
    let parse = parser::program::parse_program(&program_code);
    let parsed_program = match parse {
        Ok((_, blocks)) => blocks,
        Err(e) => {
            eprintln!("{}", e);
            Vec::new()
        }
    };
    // println!("{:#?}", parsed_program); // debug

    // evaluate
    let mut env = evaluator::environment::Env::new();
    let mut turtle = evaluator::turtle::Turtle::new(size_x, size_y);
    evaluator::eval::eval(parsed_program, &mut turtle, &mut env);

    // save
    drawing::draw::save_image(turtle, file_name);
}



fn main() {
    create_image("kwadraty", 800.0, 600.0);
}

