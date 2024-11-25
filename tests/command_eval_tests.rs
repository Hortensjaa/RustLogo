use lista7::evaluator::eval::eval_command;
use lista7::evaluator::{environment::Env, turtle::Turtle};
use lista7::parser::{unit::Unit, command::Command};

#[test]
fn test_left_rotation() {
    let mut turtle = Turtle::new(100.0, 100.0);
    let env = Env::new(); 
    eval_command(Command::Right(Unit::Val(90.0)), &mut turtle, env);
    assert_eq!(turtle.angle, 360.0); 
}

#[test]
fn test_right_rotation() {
    let mut turtle = Turtle::new(100.0, 100.0);
    let env = Env::new();
    eval_command(Command::Left(Unit::Val(90.0)), &mut turtle, env);
    assert_eq!(turtle.angle, 180.0);
}

#[test]
fn test_forward_movement() {
    let mut turtle = Turtle::new(100.0, 100.0);
    let env = Env::new();
    eval_command(Command::Forward(Unit::Val(50.0)), &mut turtle, env);
    let expected_y = 50.0 - 50.0 * 1.0;  
    assert_eq!(turtle.y, expected_y);
}

#[test]
fn test_pen_up() {
    let mut turtle = Turtle::new(100.0, 100.0);
    let env = Env::new();
    eval_command(Command::PenUp(), &mut turtle, env);
    assert_eq!(turtle.pen_down, false);
}

#[test]
fn test_pen_down() {
    let mut turtle = Turtle::new(100.0, 100.0);
    let env = Env::new();
    assert_eq!(turtle.pen_down, true);
    eval_command(Command::PenUp(), &mut turtle, env);
    assert_eq!(turtle.pen_down, false);
}

