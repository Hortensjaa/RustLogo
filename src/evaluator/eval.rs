use crate::parser::block::Condition;
use super::super::parser::unit::Unit;
use super::super::parser::command::Command;
use super::super::parser::block::Block;
use super::turtle::Turtle;
use super::environment::Env;
use rand::Rng;


pub fn eval_unit(unit: Unit, env: Env) -> f64 {
    match unit {
        Unit::Val(n) => n,
        Unit::Var(s) => 
            match env.get_var(&s) {
                Ok(val) => val, 
                Err(e) => {
                    eprintln!("Error: {}", e);
                    0.0  
                }
            },
        Unit::Exp(l, o, r) => {
            let l_val = eval_unit(*l, env.clone());
            let r_val = eval_unit(*r, env.clone());
            match o.as_str() {
                "+" => l_val + r_val,
                "-" => l_val - r_val,
                "*" => l_val * r_val,
                "/" => l_val / r_val,
                _ => {
                    eprintln!("Unknown operator: {}", o);
                    0.0
                }
            }
        },
        Unit::Random(bound) => {
            let bound_val = eval_unit(*bound, env.clone()) as u32;
            let mut rng = rand::thread_rng();
            let random_number: u32 = rng.gen_range(1..bound_val);
            random_number as f64
        }
    }
}

pub fn eval_command(command: Command, turtle: &mut Turtle, env: Env) -> Option<()>  {
    match command {
        Command::Left(unit) => {
            turtle.rotate("left", eval_unit(unit, env));
        }
        Command::Right(unit) => {
            turtle.rotate("right", eval_unit(unit, env));
        }
        Command::Forward(unit) => {
            turtle.go("forward", eval_unit(unit, env));
        }
        Command::Back(unit) => {
            turtle.go("back", eval_unit(unit, env));
        }
        Command::ClearScreen() => {
            turtle.clear_screen();
        }
        Command::PenUp() => {
            turtle.move_pen("up");
        }
        Command::PenDown() => {
            turtle.move_pen("down");
        },
        Command::SetColor(color) => {
            turtle.change_color(color);
        }
        Command::Stop() => {return None},
        _ => {}
    }
    Some(())
}


fn eval_condition(cond: Condition, env: Env) -> bool {
    let l_val = eval_unit(cond.left, env.clone());
    let r_val = eval_unit(cond.right, env.clone());
    match cond.operator.as_str() {
        "<" => l_val < r_val,
        "==" => l_val == r_val,
        ">" => l_val > r_val,
        _ => {
            eprintln!("Unknown operator: {}", cond.operator);
            false
        }
    }
}

pub fn eval_block(block: &Block, turtle: &mut Turtle, env: &mut Env) -> Option<()> {
    match block {
        Block::Single(c) => {
            if eval_command(c.clone(), turtle, env.clone()).is_none() {
                return None; 
            }
        },
        Block::If(c, instructions) => {
            if eval_condition(c.clone(), env.clone()) {
                for instruction in instructions {
                    if eval_block(instruction, turtle, env).is_none() {
                        return None; 
                    }
                }
            }
        },
        Block::Repeat(u, instructions) => {
            let n = eval_unit(u.clone(), env.clone());
            let mut i = 0.0;
            while i < n {
                for instruction in instructions {
                    if eval_block(instruction, turtle, env).is_none() {
                        return None; 
                    }
                }
                i += 1.0;
            }
        },
        Block::Function(name, _, _) => {
            env.set_fun(name.to_string(), block.clone());
        },
        Block::Call(name, args) => {
            let f = env.get_fun(name); 
                let evaluated_args: Vec<f64> = args.iter()
                .map(|arg| eval_unit(arg.clone(), env.clone())) 
                .collect();
            println!("{} called with args: {:#?}", name, evaluated_args);
            match f.unwrap() {
                Block::Function(_, params, instructions) => {
                    match env.update_many_vars(params.clone(), evaluated_args) {
                        Ok(_) => {
                            for instruction in &instructions {
                                if eval_block(instruction, turtle, env).is_none() {
                                    return None; 
                                }
                            }
                            env.remove_many_var(params);
                        }
                        Err(_) => { eprintln!("Wrong number of arguments") }
                    }
                },
                _ => { eprintln!("Not a function") }
            }
        },
    }
    Some(()) 
}


pub fn eval(blocks: Vec<Block>, turtle: &mut Turtle, env: &mut Env) {
    for block in blocks {
        if eval_block(&block, turtle, env).is_none() {
            break;
        }
    }
}
