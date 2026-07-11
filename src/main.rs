use std::env;
use crate::{calculator::calculate_expression, operators::{Operator, get_all_operators}};

mod calculator;
mod operators;

#[cfg(test)]
mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("No expression to calculate was provided.\nRun '{} help' to see available operators.", 
        args[0].split("\\").map(|sl| sl.split("/")).last().unwrap().last().unwrap()); // holy bicycle
        return; 
    }
    let expr = args[1..].join("");

    if expr.to_lowercase() == "help".to_string() {
        let mut ops = get_all_operators().values().cloned()
            .map(|op| op.name().to_owned()).collect::<Vec<String>>();
        ops.sort();

        println!("Calculates an expression provided.\nAvailable operators:\n{}.",
        ops.chunks(10).map(|chunk| chunk.join(", ")).collect::<Vec<String>>().join(", \n"));
        
        return;
    }

    let answer =  match calculate_expression(&expr) {
        Ok(ans) => ans,
        Err(e) => {
            eprintln!("{}", e);
            return;
        },
    };

    println!("{}", answer);
}