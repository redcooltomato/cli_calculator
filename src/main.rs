use std::{collections::{HashMap, VecDeque}, env};
use anyhow::{anyhow, Result};



#[derive(PartialEq, Eq, Clone, Debug)]
enum TokenSpec {
    Number,
    Operator(i8),
    Bracket,
    None,
}

impl TokenSpec {
    fn get_priority_if_operator(&self) -> Option<i8> {
        match self {
            TokenSpec::Operator(prior) => Some(*prior),
            _ => None
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Token {
    spec: TokenSpec,
    cont: String,
}

fn tokenize(expr: &String, operators: &HashMap<String, i8>) -> Result<Vec<Token>> {
    let bracks = ['(', ')'];

    #[derive(PartialEq, Clone)]
    enum BuffMode {
        Digs,
        Ops,
        None,
    }

    fn buff_mode_to_spec(buffmode: BuffMode) -> TokenSpec {
        if buffmode == BuffMode::Digs { TokenSpec::Number } 
        else if buffmode == BuffMode::Ops { TokenSpec::Operator(-1) }
        else { TokenSpec::None }
    }

    let mut buf = String::new();
    let mut buffmode: BuffMode = BuffMode::None;

    let mut tokens: Vec<Token> = Vec::new();

    for c in expr.chars().chain(" ".chars()) {
        if c == ' ' {
            if !buf.is_empty() && buffmode != BuffMode::None {
                tokens.push(Token {
                    spec: (buff_mode_to_spec(buffmode)), cont: buf
                    });
            }
            break;
        }

        if bracks.contains(&c) {
            if !buf.is_empty() {
                tokens.push(Token {
                    spec: buff_mode_to_spec(buffmode), cont: buf
                });
            }

            tokens.push(Token {
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
                        tokens.push(Token {
                            spec: buff_mode_to_spec(buffmode), cont: buf
                        });
                    }
                    buf = String::new();
                    buffmode = BuffMode::Ops;
                }
            },
            BuffMode::Ops => {
                if c.is_ascii_digit() {
                    if !buf.is_empty() {
                        tokens.push(Token {
                            spec: buff_mode_to_spec(buffmode), cont: buf
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
            tokens.push(Token {
                spec: buff_mode_to_spec(buffmode.clone()), cont: buf
            });
            buf = String::new();
        }
    }

    for elem in tokens.iter_mut() {
        if let TokenSpec::Operator(_) = elem.spec {
            if !operators.contains_key(&elem.cont) {
                return Err(anyhow!("Wrong expression"));
            } else {
                elem.spec = TokenSpec::Operator(operators.get(&elem.cont).unwrap().clone());
            }
        }
    }

    Ok(tokens)
}

fn convert_to_rpn(tokens: Vec<Token>) -> Result<Vec<Token>> {
    let mut to_parse: Vec<Token> = Vec::new();
    let mut stack: VecDeque<Token> = VecDeque::new();

    for tok in tokens {
        match tok.spec {
            TokenSpec::Number => {
                to_parse.push(tok);
            },
            TokenSpec::Bracket => {
                if tok.cont == '('.to_string() {
                    stack.push_back(tok);
                } else {
                    while !stack.is_empty() && !(stack.back().unwrap().cont == '('.to_string()) {
                        to_parse.push(stack.pop_back().unwrap());
                    }
                    if !stack.is_empty() && stack.back().unwrap().cont == '('.to_string() {
                        stack.pop_back();
                    }
                }
            },
            TokenSpec::Operator(prior) => {
                while !stack.is_empty() && stack.back().unwrap().cont != '('.to_string()
                && stack.back().unwrap().spec.get_priority_if_operator().unwrap() >= prior {
                    to_parse.push(stack.pop_back().unwrap());
                }
                stack.push_back(tok);
            },
            TokenSpec::None => {
                return Err(anyhow!("Wrong expression"));
            }
        }
    }
    while !stack.is_empty() {
        to_parse.push(stack.pop_back().unwrap());
    }

    Ok(to_parse)
}

fn parse(tokens: &Vec<Token>) -> Result<f64> {
    let mut stack: VecDeque<f64> = VecDeque::new();

    for tok in tokens {
        match tok.spec {
            TokenSpec::Number => {
                stack.push_back(tok.cont.parse().unwrap());
            },
            TokenSpec::Operator(_) => {
                match tok.cont.as_str() {
                    "+" => {
                        let (op2, op1) = (stack.pop_back().unwrap(), stack.pop_back().unwrap());
                        stack.push_back(op1 + op2);
                    },
                    "-" => {
                        let (op2, op1) = (stack.pop_back().unwrap(), stack.pop_back().unwrap());
                        stack.push_back(op1 - op2);
                    },
                    "*" => {
                        let (op2, op1) = (stack.pop_back().unwrap(), stack.pop_back().unwrap());
                        stack.push_back(op1 * op2);
                    },
                    "/" => {
                        let (op2, op1) = (stack.pop_back().unwrap(), stack.pop_back().unwrap());
                        stack.push_back(op1 / op2);
                    },
                    "sqrt" => {
                        let op1 = stack.pop_back().unwrap();
                        stack.push_back(op1.sqrt() as f64);
                    },
                    "^" => {
                        let (op2, op1) = (stack.pop_back().unwrap(), stack.pop_back().unwrap());
                        stack.push_back(op1.powf(op2));
                    }
                    _ => return Err(anyhow!("Wrong expression")),
                }
            },
            _ => return Err(anyhow!("Wrong expression")),
        }
    }

    Ok(*stack.front().unwrap())
}

fn main() {
    let operators: HashMap<String, i8> = [
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
    let expr = args[1..].join("").replace(" ", "");

    let tokens = tokenize(&expr, &operators).unwrap();
    
    let to_parse = convert_to_rpn(tokens).unwrap();

    let answer = parse(&to_parse).unwrap();

    println!("{}", answer);
}