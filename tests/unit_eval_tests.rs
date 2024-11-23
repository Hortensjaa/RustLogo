use lista7::evaluator::eval::eval_unit;
use lista7::evaluator::environment::Env;
use lista7::parser::unit::Unit;

#[test]
fn test_eval_unit_val() {
    let unit = Unit::Val(42.0);
    let env = Env::new();
    
    let result = eval_unit(unit, env);
    
    assert_eq!(result, 42.0); 
}

#[test]
fn test_eval_unit_var() {
    let mut env = Env::new();
    env.set_var("x".to_string(), Unit::Val(10.0));
    let unit = Unit::Var("x".to_string());
    
    let result = eval_unit(unit, env);
    
    assert_eq!(result, 10.0); 
}

#[test]
fn test_eval_unit_var_not_found() {
    let env = Env::new();
    let unit = Unit::Var("y".to_string());
    
    let result = eval_unit(unit, env);
    
    assert_eq!(result, 0.0); 
}

#[test]
fn test_eval_unit_exp_add() {
    let mut env = Env::new();
    env.set_var("x".to_string(), Unit::Val(10.0));
    env.set_var("y".to_string(), Unit::Val(5.0));
    
    let expr = Unit::Exp(
        Box::new(Unit::Var("x".to_string())),
        "+".to_string(),
        Box::new(Unit::Var("y".to_string())),
    );
    
    let result = eval_unit(expr, env);
    
    assert_eq!(result, 15.0);
}

#[test]
fn test_eval_unit_exp_subtract() {
    let mut env = Env::new();
    env.set_var("x".to_string(), Unit::Val(10.0));
    env.set_var("y".to_string(), Unit::Val(5.0));
    
    let expr = Unit::Exp(
        Box::new(Unit::Var("x".to_string())),
        "-".to_string(),
        Box::new(Unit::Var("y".to_string())),
    );
    
    let result = eval_unit(expr, env);
    
    assert_eq!(result, 5.0); 
}

#[test]
fn test_eval_unit_exp_multiply() {
    let mut env = Env::new();
    env.set_var("x".to_string(), Unit::Val(10.0));
    env.set_var("y".to_string(), Unit::Val(5.0));
    
    let expr = Unit::Exp(
        Box::new(Unit::Var("x".to_string())),
        "*".to_string(),
        Box::new(Unit::Var("y".to_string())),
    );
    
    let result = eval_unit(expr, env);
    
    assert_eq!(result, 50.0); 
}

#[test]
fn test_eval_unit_exp_divide() {
    let mut env = Env::new();
    env.set_var("x".to_string(), Unit::Val(10.0));
    env.set_var("y".to_string(), Unit::Val(5.0));
    
    let expr = Unit::Exp(
        Box::new(Unit::Var("x".to_string())),
        "/".to_string(),
        Box::new(Unit::Var("y".to_string())),
    );
    
    let result = eval_unit(expr, env);
    
    assert_eq!(result, 2.0);
}

#[test]
fn test_eval_unit_exp_unknown_operator() {
    let mut env = Env::new();
    env.set_var("x".to_string(), Unit::Val(10.0));
    env.set_var("y".to_string(), Unit::Val(5.0));
    
    let expr = Unit::Exp(
        Box::new(Unit::Var("x".to_string())),
        "unknown".to_string(),
        Box::new(Unit::Var("y".to_string())),
    );
    
    let result = eval_unit(expr, env);
    
    assert_eq!(result, 0.0); 
}

