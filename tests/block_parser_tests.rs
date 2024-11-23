use lista7::parser::unit::Unit;
use lista7::parser::command::Command;
use lista7::parser::block::{Block, Condition, parse_block, parse_condition};

#[test]
fn test_parse_condition() {
    assert_eq!(
        parse_condition(":size < 5"),
        Ok((
            "",
            Condition {
                left: Unit::Var("size".to_string()),
                operator: "<".to_string(),
                right: Unit::Val(5.0),
            }
        ))
    );

    assert_eq!(
        parse_condition(":x == :y"),
        Ok((
            "",
            Condition {
                left: Unit::Var("x".to_string()),
                operator: "==".to_string(),
                right: Unit::Var("y".to_string()),
            }
        ))
    );

    assert_eq!(
        parse_condition("10 > :value"),
        Ok((
            "",
            Condition {
                left: Unit::Val(10.0),
                operator: ">".to_string(),
                right: Unit::Var("value".to_string()),
            }
        ))
    );
}

#[test]
fn test_parse_repeat() {
    let input = "repeat 5 [ fd 100 rt 144 ]";
    assert_eq!(
        parse_block(input),
        Ok((
            "",
            Block::Repeat(
                Unit::Val(5.0),
                vec![
                    Block::Single(Command::Forward(Unit::Val(100.0))),
                    Block::Single(Command::Right(Unit::Val(144.0))),
                ]
            )
        ))
    );
}

#[test]
fn test_parse_if() {
    let input = "if :size > 5 [ fd 100 rt 144 ]";
    assert_eq!(
        parse_block(input),
        Ok((
            "",
            Block::If(
                Condition {
                    left: Unit::Var("size".to_string()),
                    operator: ">".to_string(),
                    right: Unit::Val(5.0),
                },
                vec![
                    Block::Single(Command::Forward(Unit::Val(100.0))),
                    Block::Single(Command::Right(Unit::Val(144.0))),
                ]
            )
        ))
    );
}

#[test]
fn test_nested_expressions() {
    let input = "if :size > 5 [ repeat 5 [ fd 100 rt 144 ] ]";
    assert_eq!(
        parse_block(input),
        Ok((
            "",
            Block::If(
                Condition {
                    left: Unit::Var("size".to_string()),
                    operator: ">".to_string(),
                    right: Unit::Val(5.0),
                },
                vec![
                    Block::Repeat(
                        Unit::Val(5.0), 
                        vec![
                            Block::Single(Command::Forward(Unit::Val(100.0))),
                            Block::Single(Command::Right(Unit::Val(144.0))),
                        ]
                    )
                ]
            )
        ))
    );
}

#[test]
fn test_parse_function_no_args() {
    let input = "to star repeat 5 [ penup fd 100 rt 144 pendown ] end";
    assert_eq!(
        parse_block(input),
        Ok((
            "",
            Block::Function(
                "star".to_string(),
                vec![], 
                vec![
                    Block::Repeat(
                        Unit::Val(5.0), 
                        vec![
                            Block::Single(Command::PenUp()),
                            Block::Single(Command::Forward(Unit::Val(100.0))),
                            Block::Single(Command::Right(Unit::Val(144.0))),
                            Block::Single(Command::PenDown()),
                        ]
                    )
                ]
            )
        ))
    );
}

#[test]fn test_parse_function_with_args() {
    let input = "to star :arg1 :arg2 repeat :size [ clearscreen rt 144 ] end";
    assert_eq!(
        parse_block(input),
        Ok((
            "",
            Block::Function(
                "star".to_string(),
                vec!["arg1".to_string(), "arg2".to_string()], 
                vec![
                    Block::Repeat(
                        Unit::Var("size".to_string()), 
                        vec![
                            Block::Single(Command::ClearScreen()),
                            Block::Single(Command::Right(Unit::Val(144.0))),
                        ]
                    )
                ]
            )
        ))
    );
}


#[test]fn test_parse_function_call_no_args() {
    let input = "star";
    assert_eq!(
        parse_block(input),
        Ok((
            "",
            Block::Call(
                "star".to_string(),
                vec![]
        )))
    );
}


#[test]fn test_parse_function_call_args() {
    let input = "star 30 :size";
    assert_eq!(
        parse_block(input),
        Ok((
            "",
            Block::Call(
                "star".to_string(),
                vec![Unit::Val(30.0), Unit::Var("size".to_string())]
        )))
    );
}


