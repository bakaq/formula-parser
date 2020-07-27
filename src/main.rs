use std::io;
use std::io::Write;

mod scanner;

fn main() {
    print!("Enter the formula: ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let token_stream = scanner::scan(&input.trim());
    
    println!("{:#?}", token_stream);
}
