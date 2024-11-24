use std::collections::HashMap;

use super::super::parser::unit::Unit;
use super::super::parser::block::Block;

// wrapper for local environment inside block
#[derive(Debug, PartialEq, Clone)]
pub struct Env {
    vars: HashMap<String, Unit>,
    functions: HashMap<String, Block>
}

impl Env {
    pub fn new() -> Self {
        Env {
            vars: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn get_var(&self, var_name: &str) -> Result<Unit, String>  {
        match self.vars.get(var_name) {
            Some(value) => Ok(value.clone()),
            None => Err("Value {} not in environment".to_string()),
        }
    }

    pub fn set_var(&mut self, var_name: String, val: Unit) {
        self.vars.insert(var_name, val);
    }

    pub fn update_many_vars(&mut self, params: Vec<String>, args: Vec<Unit>) -> Result<(), String> {
        if params.len() != args.len() {
            return Err("Number of parameters and arguments do not match".to_string());
        }

        for (param, arg) in params.into_iter().zip(args.into_iter()) {
            self.vars.insert(param, arg);
        }

        Ok(())
    }

    pub fn remove_var(&mut self, var_name: &str) -> Option<Unit> {
        self.vars.remove(var_name)
    }

    pub fn remove_many_var(&mut self, params: Vec<String>) -> Vec<Option<Unit>> {
        params.into_iter().map(|key| self.vars.remove(&key)).collect()
    }

    pub fn get_fun(&self, fun_name: &str) -> Result<Block, String> {
        match self.functions.get(fun_name) {
            Some(value) => Ok(value.clone()),
            None => Err("Function definition not in environment".to_string()),
        }
    }

    pub fn set_fun(&mut self, fun_name: String, block: Block) {
        self.functions.insert(fun_name, block);
    }

}