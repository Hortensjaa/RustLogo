use lista7::parser::{command::Command, unit::Unit};
use lista7::parser::block::{Block, Condition};
use lista7::evaluator::{environment::Env, turtle::Turtle, eval::eval_block};

fn setup_env() -> Env {
    let mut env = Env::new();
    env.set_var("x".to_string(), 100.0);
    env
}

#[test]
fn test_single_block() {
    let mut turtle = Turtle::new(800.0, 600.0);
    let mut env = setup_env();
    let block = Block::Single(Command::Forward(Unit::Val(50.0)));

    eval_block(&block, &mut turtle, &mut env);

    assert_eq!(turtle.y, 350.0); 
    assert_eq!(turtle.lines.len(), 1);
}

#[test]
fn test_if_block() {
    let mut turtle = Turtle::new(800.0, 600.0);
    let mut env = setup_env();
    let condition = Condition {
        left: Unit::Val(50.0),
        right: Unit::Val(100.0),
        operator: "<".to_string(),
    };
    let instructions = vec![Block::Single(Command::Forward(Unit::Val(50.0)))];
    let block = Block::If(condition, instructions);

    eval_block(&block, &mut turtle, &mut env);

    assert_eq!(turtle.y, 350.0);
    assert_eq!(turtle.lines.len(), 1);
}

#[test]
fn test_repeat_block() {
    let mut turtle = Turtle::new(800.0, 600.0);
    let mut env = setup_env();
    let instructions = vec![Block::Single(Command::Forward(Unit::Val(10.0)))];
    let block = Block::Repeat(Unit::Val(5.0), instructions);

    eval_block(&block, &mut turtle, &mut env);

    assert_eq!(turtle.y, 350.0);
    assert_eq!(turtle.lines.len(), 5);
}

#[test]
fn test_function_and_call_block() {
    let mut turtle = Turtle::new(800.0, 600.0);
    let mut env = setup_env();

    // definition
    let function = Block::Function(
        "draw_line".to_string(),
        vec!["distance".to_string()],
        vec![Block::Single(Command::Forward(Unit::Var("distance".to_string())))],
    );
    eval_block(&function, &mut turtle, &mut env);

    // call
    let call = Block::Call(
        "draw_line".to_string(),
        vec![Unit::Val(100.0)],
    );
    eval_block(&call, &mut turtle, &mut env);

    assert_eq!(turtle.y, 400.0);
    assert_eq!(turtle.lines.len(), 1);
}

#[test]
fn test_eval_block_function_no_args() {
    let mut turtle = Turtle::new(800.0, 600.0);
    let mut env = Env::new();

    let func_block = Block::Function(
        "draw_square".to_string(),
        vec![], 
        vec![
            Block::Single(Command::Forward(Unit::Val(50.0))),
            Block::Single(Command::Right(Unit::Val(90.0))),
            Block::Single(Command::Forward(Unit::Val(50.0))),
            Block::Single(Command::Right(Unit::Val(90.0))),
            Block::Single(Command::Forward(Unit::Val(50.0))),
            Block::Single(Command::Right(Unit::Val(90.0))),
            Block::Single(Command::Forward(Unit::Val(50.0))),
        ],
    );

    eval_block(&func_block, &mut turtle, &mut env);

    let call_block = Block::Call("draw_square".to_string(), vec![]);
    eval_block(&call_block, &mut turtle, &mut env);

    assert_eq!(turtle.x, 400.0); 
    assert_eq!(turtle.y, 300.0);
    assert_eq!(turtle.lines.len(), 4);
}


#[test]
fn test_if_block_false_condition() {
    let mut turtle = Turtle::new(800.0, 600.0);
    let mut env = setup_env();
    let condition = Condition {
        left: Unit::Val(150.0),
        right: Unit::Val(100.0),
        operator: "<".to_string(),
    };
    let instructions = vec![Block::Single(Command::Forward(Unit::Val(50.0)))];
    let block = Block::If(condition, instructions);

    eval_block(&block, &mut turtle, &mut env);

    assert_eq!(turtle.x, 400.0); 
    assert_eq!(turtle.y, 300.0); 
    assert_eq!(turtle.lines.len(), 0);
}

#[test]
fn test_call_block_wrong_arguments() {
    let mut turtle = Turtle::new(800.0, 600.0);
    let mut env = setup_env();

    let function = Block::Function(
        "draw_line".to_string(),
        vec!["distance".to_string()],
        vec![Block::Single(Command::Forward(Unit::Var("distance".to_string())))],
    );
    eval_block(&function, &mut turtle, &mut env);

    let call = Block::Call("draw_line".to_string(), vec![]);
    eval_block(&call, &mut turtle, &mut env);

    assert_eq!(turtle.y, 300.0); 
    assert_eq!(turtle.lines.len(), 0);
}




