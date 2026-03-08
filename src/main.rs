use std::{
    env,
    collections::VecDeque,
};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(i64),
    Operator(String),
    Undefined,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BufferMode {
    Number,
    Operator,
    Empty,
}

fn main() {
    let args: Vec<String> = env::args().collect();


    if (args.len() <= 1) {
        return;
        // todo: do --help stuff, prolly run this when expression is invalid
    }


    let expr: String = args[1..].join("");
    let mut to_parse: Vec<Token> = Vec::new();
    let mut buffer: String = String::new();
    let mut buffer_mode: BufferMode = BufferMode::Empty;

    for char in expr.chars() {
        match char {
            '(' | ')' => {
                to_parse.push(Token::Operator(char.to_string()));
            },
            '0'..='9' => {
                if buffer_mode != BufferMode::Number && buffer_mode != BufferMode::Empty {
                    to_parse.push(Token::Operator(buffer));
                    buffer = String::new();
                }
                buffer.push(char);
                buffer_mode = BufferMode::Number;
            },
            _ => {
                if buffer_mode != BufferMode::Operator && buffer_mode != BufferMode::Empty {
                    to_parse.push(Token::Number(buffer.parse().unwrap())); // unsafe as hell
                    buffer.clear();
                }
                buffer.push(char);
                buffer_mode = BufferMode::Operator;
            },
        }
    }

    let mut calculations_queue: VecDeque<i64> = VecDeque::new();
    for token in to_parse {
        // wait of the world
    }
}