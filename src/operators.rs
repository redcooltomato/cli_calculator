use std::{collections::HashMap};

use anyhow::{anyhow, Result};

pub struct Operator {
    name: String,
    precedence: i8,
    arity: u8,
    operate_fn: fn(Vec<f64>) -> Result<f64>,
}

impl Operator {
    pub fn name(&self) -> &str { &self.name }
    pub fn precedence(&self) -> i8 { self.precedence }
    pub fn arity(&self) -> u8 { self.arity }

    pub fn operate(&self, args: Vec<f64>) -> Result<f64> {
        if args.len() != self.arity as usize {
            return Err(anyhow!("Wrong ammount of arguments"));
        }

        (self.operate_fn)(args)
    }
}

pub fn get_all_operators() -> HashMap<String, Operator> {
    let operators: Vec<Operator> = Vec::from([
        Operator { name: "+".to_string(), precedence: 1, arity: 2, operate_fn: |args| {
            Ok(args[0] + args[1])
        }},
        Operator { name: "-".to_string(), precedence: 1, arity: 2, operate_fn: |args| {
            Ok(args[0] - args[1])
        }},
        Operator { name: "*".to_string(), precedence: 2, arity: 2, operate_fn: |args| {
            Ok(args[0] * args[1])
        }},
        Operator { name: "/".to_string(), precedence: 2, arity: 2, operate_fn: |args| {
            if args[1] == 0.0 { Err(anyhow!("Division by zero")) }
            else { Ok(args[0] / args[1]) }
        }},
        Operator { name: "rt".to_string(), precedence: 3, arity: 2, operate_fn: |args| {
            if args[1] < 0.0 && (args[0] % 2.0 == 0.0) { Err(anyhow!("Can't get an even root of a negative number")) }
            else { Ok(args[1].powf(1.0 / args[0])) }
        }},
        Operator { name: "sqrt".to_string(), precedence: 3, arity: 1, operate_fn: |args| {
            if args[1] < 0.0 { Err(anyhow!("Can't get a square root of a negative number")) }
            else { Ok(args[1].powf(1.0 / args[0])) }
        }},
        Operator { name: "^".to_string(), precedence: 3, arity: 2, operate_fn: |args| {
            Ok(args[0].powf(args[1]))
        }},
        Operator { name: "sin".to_string(), precedence: 3, arity: 1, operate_fn: |args| {
            Ok(args[0].to_radians().sin())
        }},
        Operator { name: "cos".to_string(), precedence: 3, arity: 1, operate_fn: |args| {
            Ok(args[0].to_radians().cos())
        }},
        Operator { name: "tan".to_string(), precedence: 3, arity: 1, operate_fn: |args| {
            Ok(args[0].to_radians().tan())
        }},
    ]);

    let mut result: HashMap<String, Operator> = HashMap::new();
    
    for op in operators {
        result.insert(op.name.clone(), op);
    }

    result
}