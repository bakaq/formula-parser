use std::io;
use std::io::Write;

mod scanner;
mod parser;
mod interpreter;

fn main() {
    print!("Enter the formula: ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    println!("{}", interpreter::interpret(&input));
}
