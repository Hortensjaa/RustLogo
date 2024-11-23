use lista7::parser::unit::Unit;
use lista7::parser::command::{Command, parse_command};

#[test]
fn test_parse_forward() {
    let test_cases = vec![
        ("FORWARD 100", Command::Forward(Unit::Val(100.0))),
        ("fd 200", Command::Forward(Unit::Val(200.0))),
        ("FD 300", Command::Forward(Unit::Val(300.0))),
        ("forward 200.01", Command::Forward(Unit::Val(200.01))),
    ];

    for (input, expected) in test_cases {
        let result = parse_command(input);
        match result {
            Ok((_, command)) => assert_eq!(command, expected),
            Err(_) => panic!("Błąd parsowania dla '{}'", input),
        }
    }
}

#[test]
fn test_parse_commands() {
    let test_cases = vec![
        ("lt 100", Command::Left(Unit::Val(100.0))),
        ("rt :size", Command::Right(Unit::Var("size".to_string()))),
        ("rIgHt :size * 3", 
            Command::Right(
                Unit::Exp(
                    Box::new(Unit::Var("size".to_string())),
                    "*".to_string(),
                    Box::new(Unit::Val(3.0))))),
        ("BACK :size / :times", 
            Command::Back(
                Unit::Exp(
                    Box::new(Unit::Var("size".to_string())),
                    "/".to_string(),
                    Box::new(Unit::Var("times".to_string()))))),
        ("wait 20 + 4.9", 
            Command::Wait(
                Unit::Exp(
                    Box::new(Unit::Val(20.0)),
                    "+".to_string(),
                    Box::new(Unit::Val(4.9))))),
    ];

    for (input, expected) in test_cases {
        let result = parse_command(input);
        match result {
            Ok((_, command)) => assert_eq!(command, expected),
            Err(_) => panic!("Błąd parsowania dla '{}'", input),
        }
    }
}

#[test]
fn test_parse_no_args() {
    let test_cases = vec![
        ("clearscreen", Command::ClearScreen()),
        ("PENUP", Command::PenUp()),
        ("penDown", Command::PenDown()),
    ];

    for (input, expected) in test_cases {
        let result = parse_command(input);
        match result {
            Ok((_, command)) => assert_eq!(command, expected),
            Err(_) => panic!("Błąd parsowania dla '{}'", input),
        }
    }
}

#[test]
fn test_parse_invalid() {
    let invalid_inputs = vec!["idk 200", "fd-100", "fd", "FORWARD"];

    for input in invalid_inputs {
        let result = parse_command(input);
        assert!(result.is_err(), "Oczekiwano błędu dla '{}', ale parsowanie zakończyło się sukcesem", input);
    }
}
