use std::{
    env,
    collections::VecDeque,
};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(i64),
    Operator(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BufferMode {
    Number,
    Operator,
    Empty,
}

fn tokenize(expr: &str) -> VecDeque<Token> { // todo add actual tokens (math functions matcher or smth)
    let mut to_parse: VecDeque<Token> = VecDeque::new();
    let mut buffer: String = String::new();
    let mut buffer_mode: BufferMode = BufferMode::Empty;

    for char in expr.chars() {
        match char {
            '(' | ')' => {
                to_parse.push_back(Token::Operator(char.to_string()));
            },
            '0'..='9' => {
                if buffer_mode == BufferMode::Operator {
                    to_parse.push_back(Token::Operator(buffer));
                    buffer = String::new();
                }
                buffer.push(char);
                buffer_mode = BufferMode::Number;
            },
            _ => {
                if buffer_mode == BufferMode::Number {
                    to_parse.push_back(Token::Number(buffer.parse().unwrap())); // unsafe as hell
                    buffer.clear();
                }
                buffer.push(char);
                buffer_mode = BufferMode::Operator;
            },
        }
    }

    if !buffer.is_empty() {
        match buffer_mode {
            BufferMode::Number => {
                to_parse.push_back(Token::Number(buffer.parse().unwrap()));
            },
            BufferMode::Operator => {
                to_parse.push_back(Token::Operator(buffer));
            }
            _ => (),
        }
    }

    to_parse
}

fn main() {
    let args: Vec<String> = env::args().collect();


    if args.len() <= 1 {
        return;
        // todo: do --help stuff, prolly run this when expression is invalid
    }


    let expr: String = args[1..].join("");
    let mut to_parse: VecDeque<Token> = tokenize(&expr);

    let mut calculations_queue: VecDeque<i64> = VecDeque::new();
    for token in to_parse {
        println!("{:?}", token);
    }
}