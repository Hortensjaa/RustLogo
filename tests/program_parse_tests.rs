use lista7::parser::{unit::Unit, command::Command, block::Block, program::parse_program};

use std::fs;

pub fn read_from_file(file_name: &str) -> String {
    let filename = format!("codes/{}.txt", file_name);
    let program_code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_name, e);
            return "error".to_string();
        }
    };
    program_code.replace("\r\n\t", " ").trim().to_string()
}


#[test]
fn test_parse_program() {
    let program = "
    to star 
    repeat 5 [ fd 100 rt 144 ] 
    end 
    clearscreen 
    star";

    let expected = vec![
        Block::Function(
            "star".to_string(),
            vec![],
            vec![
                Block::Repeat(
                    Unit::Val(5.0),
                    vec![
                        Block::Single(Command::Forward(Unit::Val(100.0))),
                        Block::Single(Command::Right(Unit::Val(144.0))),
                    ],
                ),
            ],
        ),
        Block::Single(Command::ClearScreen()),
        Block::Call("star".to_string(), vec![]),
    ];

    match parse_program(program) {
        Ok((remaining, blocks)) => {
            assert!(remaining.is_empty(), "Remaining input should be empty.");
            assert_eq!(blocks, expected, "Parsed blocks do not match the expected structure.");
        }
        Err(e) => {
            panic!("Error parsing program: {:?}", e);
        }
    }
}

#[test]
fn test_parse_star() {
    let program = &read_from_file("star");

    let expected = vec![
        Block::Function(
            "star".to_string(),
            vec![],
            vec![
                Block::Repeat(
                    Unit::Val(5.0),
                    vec![
                        Block::Single(Command::Forward(Unit::Val(100.0))),
                        Block::Single(Command::Right(Unit::Val(144.0))),
                    ],
                ),
            ],
        ),
        Block::Single(Command::ClearScreen()),
        Block::Call("star".to_string(), vec![]),
    ];

    match parse_program(program) {
        Ok((remaining, blocks)) => {
            assert!(remaining.is_empty(), "Remaining input should be empty.");
            assert_eq!(blocks, expected, "Parsed blocks do not match the expected structure.");
        }
        Err(e) => {
            panic!("Error parsing program: {:?}", e);
        }
    }
}

// for the rest i just show, that parsing doesnt end with an error
#[test]
fn test_parse_squares() {
    let program = &read_from_file("squares");

    match parse_program(program) {
        Ok((remaining, _)) => {
            assert!(remaining.is_empty(), "Remaining input should be empty.");
        }
        Err(e) => {
            panic!("Error parsing program: {:?}", e);
        }
    }
}
#[test]
fn test_parse_race() {
    let program = &read_from_file("race");

    match parse_program(program) {
        Ok((remaining, _)) => {
            assert!(remaining.is_empty(), "Remaining input should be empty.");
        }
        Err(e) => {
            panic!("Error parsing program: {:?}", e);
        }
    }
}
#[test]
fn test_parse_fern() {
    let program = &read_from_file("fern");

    match parse_program(program) {
        Ok((remaining, _)) => {
            assert!(remaining.is_empty(), "Remaining input should be empty.");
        }
        Err(e) => {
            panic!("Error parsing program: {:?}", e);
        }
    }
}
#[test]
fn test_parse_tree() {
    let program = &read_from_file("tree");

    match parse_program(program) {
        Ok((remaining, _)) => {
            assert!(remaining.is_empty(), "Remaining input should be empty.");
        }
        Err(e) => {
            panic!("Error parsing program: {:?}", e);
        }
    }
}

