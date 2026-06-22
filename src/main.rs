use std::env;
use crate::calculator::calculate_expression;

mod calculator;
mod operators;

#[cfg(test)]
mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("no args!");
        return; 
    }
    let expr = args[1..].join("");

    let answer =  match calculate_expression(&expr) {
        Ok(ans) => ans,
        Err(e) => {
            eprintln!("{}", e);
            return;
        },
    };

    println!("{}", answer);
}