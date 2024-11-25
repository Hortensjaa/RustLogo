use std::collections::HashMap;

use super::super::parser::block::Block;

// wrapper for local environment inside block
#[derive(Debug, PartialEq, Clone)]
pub struct Env {
    vars: HashMap<String, Vec<f64>>,
    functions: HashMap<String, Block>
}

impl Env {
    pub fn new() -> Self {
        Env {
            vars: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn get_var(&self, var_name: &str) -> Result<f64, String>  {
        match self.vars.get(var_name) {
            Some(stack) => Ok(*stack.last().ok_or_else(|| format!("Value {} not in environment", var_name))?),
            None => Err(format!("Value {} not in environment", var_name))
        }
    }

    pub fn set_var(&mut self, var_name: String, val: f64) {
        self.vars.entry(var_name).or_insert_with(Vec::new).push(val);
    }

    pub fn update_many_vars(&mut self, params: Vec<String>, args: Vec<f64>) -> Result<(), String> {
        if params.len() != args.len() {
            return Err("Number of parameters and arguments do not match".to_string());
        }

        for (param, arg) in params.into_iter().zip(args.into_iter()) {
            self.set_var(param, arg);
        }

        Ok(())
    }

    pub fn pop_var(&mut self, var_name: &str) -> Option<f64> {
        self.vars.get_mut(var_name).and_then(|stack| stack.pop())
    }

    pub fn pop_many_vars(&mut self, params: Vec<String>) -> Vec<Option<f64>> {
        params.into_iter().map(|key| self.pop_var(&key)).collect()
    }

    pub fn get_fun(&self, fun_name: &str) -> Result<Block, String> {
        match self.functions.get(fun_name) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("Function '{}' not found in environment", fun_name)),
        }
    }

    pub fn set_fun(&mut self, fun_name: String, block: Block) {
        self.functions.insert(fun_name, block);
    }

}