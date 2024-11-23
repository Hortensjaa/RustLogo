use lista7::parser::{unit::Unit, command::Command, block::Block, program::parse_program};


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

