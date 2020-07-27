use std::io;
use std::io::Write;

mod scanner;
mod parser;

fn main() {
    print!("Enter the formula: ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let token_stream = scanner::scan(&input.trim());
    
    let (ast, _) = parser::binexpr(&token_stream, 0);
    
    println!("{:#?}", ast);
}
