use lista7::parser::unit::{Unit, parse_unit};

#[test]
fn test_parse_val() {
    let test_cases = vec![
        ("100.57", Unit::Val(100.57)),
        ("67", Unit::Val(67.0)),
        ("-67", Unit::Val(-67.0)),
        ("-67.96", Unit::Val(-67.96)),
    ];

    for (input, expected) in test_cases {
        let result = parse_unit(input);
        match result {
            Ok((_, unit)) => assert_eq!(unit, expected),
            Err(_) => panic!("Błąd parsowania dla '{}'", input),
        }
    }
}

#[test]
fn test_parse_var() {
    let test_cases = vec![
        (":size", Unit::Var("size".to_string())),
        (":times", Unit::Var("times".to_string())),
    ];

    for (input, expected) in test_cases {
        let result = parse_unit(input);
        match result {
            Ok((_, unit)) => assert_eq!(unit, expected),
            Err(_) => panic!("Błąd parsowania dla '{}'", input),
        }
    }
}

#[test]
fn test_parse_random() {
    let test_cases = vec![
        ("random 200", Unit::Random(Box::new(Unit::Val(200.0)))),
        ("Rm :times", Unit::Random(Box::new(Unit::Var("times".to_string())))),
    ];

    for (input, expected) in test_cases {
        let result = parse_unit(input);
        match result {
            Ok((_, unit)) => assert_eq!(unit, expected),
            Err(_) => panic!("Błąd parsowania dla '{}'", input),
        }
    }
}

#[test]
fn test_parse_expression() {
    let test_cases = vec![
        ("9.5 / -3", Unit::Exp(
            Box::new(Unit::Val(9.5)),
            "/".to_string(),
            Box::new(Unit::Val(-3.0))
        )),
        (":size * 3", Unit::Exp(
            Box::new(Unit::Var("size".to_string())),
            "*".to_string(),
            Box::new(Unit::Val(3.0))
        )),
        ("-2.14 + :times", Unit::Exp(
            Box::new(Unit::Val(-2.14)),
            "+".to_string(),
            Box::new(Unit::Var("times".to_string()))
        )),
        (":iterations - :times", Unit::Exp(
            Box::new(Unit::Var("iterations".to_string())),
            "-".to_string(),
            Box::new(Unit::Var("times".to_string()))
        )),
    ];

    for (input, expected) in test_cases {
        let result = parse_unit(input);
        match result {
            Ok((_, unit)) => assert_eq!(unit, expected),
            Err(_) => panic!("Błąd parsowania dla '{}'", input),
        }
    }
}
