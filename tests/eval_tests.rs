use lista7::parser::{command::Command, unit::Unit};
use lista7::parser::block::Block;
use lista7::evaluator::{environment::Env, turtle::Turtle, eval::eval};

#[test]
fn test_program_evaluation() {
    let parsed_program = vec![
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

    let mut env = Env::new();
    let mut turtle = Turtle::new(800.0, 600.0);

    eval(parsed_program, &mut turtle, &mut env);

    assert_eq!(turtle.x, 400.0);
    assert_eq!(turtle.y, 300.0);
    assert_eq!(turtle.lines.len(), 5); 
}