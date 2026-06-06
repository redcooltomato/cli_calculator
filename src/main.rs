use std::{collections::{HashMap, HashSet}, env};
use anyhow::{anyhow, Result};



#[derive(PartialEq, Eq, Clone, Debug)]
enum TokenSpec {
    Number,
    Operator(i8),
    Bracket,
    None,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Token {
    spec: TokenSpec,
    cont: String,
}

fn split(expr: String, operators: &HashMap<String, i8>) -> Result<Vec<Token>> {
    let bracks = ['(', ')'];

    #[derive(PartialEq, Clone)]
    enum BuffMode {
        Digs,
        Ops,
        None,
    }

    fn buffmodeToSpec(buffmode: BuffMode) -> TokenSpec {
        if buffmode == BuffMode::Digs { TokenSpec::Number } 
        else if buffmode == BuffMode::Ops { TokenSpec::Operator(-1) }
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
        
        if buffmode == BuffMode::Ops && operators.contains_key(&buf) {
            result.push(Token {
                spec: buffmodeToSpec(buffmode.clone()), cont: buf
            });
            buf = String::new();
        }
    }

    for elem in result.iter_mut() {
        if let TokenSpec::Operator(priority) = elem.spec {
            if !operators.contains_key(&elem.cont) {
                return Err(anyhow!("Unknow operator found. Expression likely contains errors.".to_string()));
            } else {
                elem.spec = TokenSpec::Operator(operators.get(&elem.cont).unwrap().clone());
            }
        }
    }

    Ok(result)
}

fn main() {
    let mut operators: HashMap<String, i8> = [
        ("+", 1),
        ("-", 1),
        ("*", 2),
        ("/", 2),
        ("^", 3),
        ("sqrt", 3),
    ].into_iter().map(|(k, v)| { (k.to_string(), v) }).collect();

    let args : Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("no args!");
        return; 
    }
    let args = args[1..].join("").replace(" ", "");

    let mut items = split(args, &operators);
    print!("{:?}", items);
}