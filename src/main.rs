use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let subject: String = args[1..].join("");

    println!("{}", subject);
}