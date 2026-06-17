use std::collections::{HashMap, VecDeque};
use anyhow::{anyhow, Result};

use crate::operators::{Operator, get_all_operators};

pub const DEFAULT_ERROR_MSG: &str = "Invalid expression";

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

fn tokenize(expr: &String, operators: &HashMap<String, Operator>) -> Result<Vec<Token>> {
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
                if !c.is_ascii_digit() || c == '.' {
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
                if c.is_ascii_digit() || c == '.' {
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

    let mut new_tokens: Vec<Token> = Vec::new();
    for i in 0..(tokens.len() - 1) {
        if &tokens[i].cont == "-" && tokens[i + 1].spec == TokenSpec::Number {
            tokens[i + 1].cont.insert(0, '-');
        } else {
            new_tokens.push(tokens[i].clone());
        }
    }
    new_tokens.push(tokens.last().cloned().unwrap());
    tokens = new_tokens;

    for elem in tokens.iter_mut() {
        if let TokenSpec::Operator(_) = elem.spec {
            if !operators.contains_key(&elem.cont) {
                return Err(anyhow!(DEFAULT_ERROR_MSG));
            } else {
                elem.spec = TokenSpec::Operator(operators.get(&elem.cont).unwrap().precedence());
            }
        }
    }

    Ok(tokens)
}

fn convert_to_rpn(tokens: Vec<Token>) -> Result<Vec<Token>> {
    let mut to_parse: Vec<Token> = Vec::new();
    let mut stack: VecDeque<Token> = VecDeque::new();
    let left_par = "(".to_owned(); // it is used too often here, idk


    for tok in tokens {
        match tok.spec {
            TokenSpec::Number => {
                to_parse.push(tok);
            },
            TokenSpec::Bracket => {
                if tok.cont == left_par.to_owned() {
                    stack.push_back(tok);
                } else {
                    while !stack.is_empty() && !(stack.back().unwrap().cont == left_par.to_owned()) {
                        to_parse.push(stack.pop_back().unwrap());
                    }
                    if !stack.is_empty() && stack.back().unwrap().cont == left_par.to_owned() {
                        stack.pop_back();
                    }
                }
            },
            TokenSpec::Operator(prior) => {
                while !stack.is_empty() && stack.back().unwrap().cont != left_par.to_owned()
                && stack.back().unwrap().spec.get_priority_if_operator().unwrap() >= prior {
                    to_parse.push(stack.pop_back().unwrap());
                }
                stack.push_back(tok);
            },
            TokenSpec::None => {
                return Err(anyhow!(DEFAULT_ERROR_MSG));
            }
        }
    }
    while !stack.is_empty() {
        to_parse.push(stack.pop_back().unwrap());
    }

    Ok(to_parse)
}

fn parse(tokens: &Vec<Token>, operators: &HashMap<String, Operator>) -> Result<f64> {
    fn get_args(stack: &mut VecDeque<f64>, arg_count: u8) -> Result<Vec<f64>> {
        match arg_count {
            0 => return Ok(vec![] as Vec<f64>),
            1 => {
                let arg = stack.pop_back();
                if arg.is_some() {
                    return Ok(vec![arg.unwrap()]);
                } else {
                    return Err(anyhow!(DEFAULT_ERROR_MSG));
                }
            },
            2 => {
                let (arg2, arg1) = (stack.pop_back(), stack.pop_back());
                if arg1.is_some() && arg2.is_some() {
                    return Ok(vec![arg1.unwrap(), arg2.unwrap()]);
                } else {
                    return Err(anyhow!(DEFAULT_ERROR_MSG));
                }
            },
            _ => {
                unreachable!();
            },
        }
    }

    let mut stack: VecDeque<f64> = VecDeque::new();

    for tok in tokens {
        match tok.spec {
            TokenSpec::Number => {
                let num = tok.cont.parse();
                if num.is_err() { return Err(anyhow!(DEFAULT_ERROR_MSG)); }
                stack.push_back(num.unwrap());
            },
            TokenSpec::Operator(_) => {
                if !operators.contains_key(&tok.cont) {
                    return Err(anyhow!(DEFAULT_ERROR_MSG));
                } else {
                    let op = operators.get(&tok.cont).unwrap();

                    let args = get_args(&mut stack, op.arity());
                    if args.is_err() {
                        return Err(args.unwrap_err());
                    }

                    let res = op.operate(args.unwrap());
                    if res.is_err() {
                        return Err(res.unwrap_err());
                    }
                    stack.push_back(res.unwrap());
                }
            },
            _ => return Err(anyhow!(DEFAULT_ERROR_MSG)),
        }
    }

    if stack.is_empty() {
        return Err(anyhow!(DEFAULT_ERROR_MSG));
    }
    Ok(*stack.front().unwrap())
}

pub fn calculate_expression(expr: &String) -> Result<f64> {
    let operators: HashMap<String, Operator> = get_all_operators();

    let tokens = tokenize(&expr, &operators);
    if tokens.is_err() {
        return Err(tokens.unwrap_err());
    }
    let tokens = tokens.unwrap();
    
    let to_parse = convert_to_rpn(tokens);
    if to_parse.is_err() {
        return Err(to_parse.unwrap_err());
    }
    let to_parse = to_parse.unwrap();
    
    let answer = parse(&to_parse, &operators);
    if answer.is_err() {
        return Err(answer.unwrap_err());
    }
    let answer = answer.unwrap();

    Ok(answer)
}