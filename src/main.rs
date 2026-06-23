use std::env;
use crate::calculator::calculate_expression;

mod calculator;
mod operators;

#[cfg(test)]
mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("No expression to calculate was provided.\nRun '{} help' to see available operators.", 
        args[0].split("\\").map(|sl| sl.split("/")).last().unwrap().last().unwrap());
        return; 
    }
    let expr = args[1..].join("");

    if expr.to_lowercase() == "help".to_string() {
        println!("Calculates an expression provided.\nAvailable operators:\nx + y, x - y, x * y, x / y,\nx ^ y, sqrt x, y rt x,\nsin x, cos x, tan x, asin x, acos x, atan x.");
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