use std::env;
use crate::calculator::calculate_expression;

mod calculator;


fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("no args!");
        return; 
    }
    let expr = args[1..].join("").replace(" ", "");

    let answer = calculate_expression(&expr);
    if answer.is_err() {
        println!("{}", answer.unwrap_err()); return;
    }
    let answer = answer.unwrap();

    println!("{}", answer);
}