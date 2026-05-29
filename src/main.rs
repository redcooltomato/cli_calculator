use std::{env, collections::HashSet};


fn split(expr: String) -> Vec<String> {
    let blocks = ["+", "-", "*", "/", "(", ")"];
    let hs = HashSet::from(blocks);

    #[derive(PartialEq)]
    enum BuffMode {
        Digs,
        Ops,
        None,
    }
    let mut buf = String::new();
    let mut buffmode: BuffMode = BuffMode::None;

    let mut result: Vec<String> = Vec::new();
    for c in expr.chars().chain(" ".chars()) {
        if c == ' ' {
            result.push(buf.clone());
            break;
        }
        match buffmode {
            BuffMode::Digs => {
                if !c.is_ascii_digit() {
                    result.push(buf.clone());
                    buf.clear();
                    buffmode = BuffMode::Ops;
                }
            },
            BuffMode::Ops => {
                if c.is_ascii_digit() {
                    result.push(buf.clone());
                    buf.clear();
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