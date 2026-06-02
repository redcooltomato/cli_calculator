use std::{env, collections::HashSet};



#[derive(PartialEq, Eq, Clone, Debug)]
enum TokenSpec {
    Number,
    Operator,
    Bracket,
    None,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Token {
    spec: TokenSpec,
    cont: String,
}

fn split(expr: String) -> Vec<Token> {
    println!("{:?}", expr);
    let bracks = ['(', ')'];

    #[derive(PartialEq)]
    enum BuffMode {
        Digs,
        Ops,
        None,
    }

    fn buffmodeToSpec(buffmode: BuffMode) -> TokenSpec {
        if buffmode == BuffMode::Digs { TokenSpec::Number } 
        else if buffmode == BuffMode::Ops { TokenSpec::Operator }
        else { TokenSpec::None }
    }

    let mut buf = String::new();
    let mut buffmode: BuffMode = BuffMode::None;

    let mut result: Vec<Token> = Vec::new();
    for c in expr.chars().chain(" ".chars()) {
        if c == ' ' {
            if !buf.is_empty() && buffmode != BuffMode::None {
                result.push(Token {
                     spec: (buffmodeToSpec(buffmode)), cont: buf
                    });
            }
            break;
        }

        if bracks.contains(&c) {
            if !buf.is_empty() {
                result.push(Token {
                     spec: buffmodeToSpec(buffmode), cont: buf
                    });
                buf = String::new();
            }

            result.push(Token {
                 spec: TokenSpec::Bracket, cont: c.to_string()
                });
                buffmode = BuffMode::None;
            buf = String::new();
            continue;
        }

        match buffmode {
            BuffMode::Digs => {
                if !c.is_ascii_digit() {
                    if !buf.is_empty() {
                        result.push(Token {
                     spec: buffmodeToSpec(buffmode), cont: buf
                    });
                    }
                    buf = String::new();
                    buffmode = BuffMode::Ops;
                }
            },
            BuffMode::Ops => {
                if c.is_ascii_digit() {
                    if !buf.is_empty() {
                        result.push(Token {
                     spec: buffmodeToSpec(buffmode), cont: buf
                    });
                    }
                    buf = String::new();
                    buffmode = BuffMode::Digs;
                }
            },
            BuffMode::None => {
                if c.is_ascii_digit() {
                    buffmode = BuffMode::Digs;
                } else {
                    buffmode = BuffMode::Ops;
                }
            }
        }
        buf.push(c);
    }

    result
}

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("no args!");
        return; 
    }

    let args = args[1..].join("");
    let mut items = split(args);
    print!("{:?}", items);
}