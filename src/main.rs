mod lexer;
mod parser;
mod token;
use lexer::Lexer;
use parser::Parser;
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();
        if input == "" {
            break;
        }
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        match parser.parse_expression() {
            Ok(value) => println!("{value}"),
            Err(message) => println!("ERROR: {message}"),
        }
    }
}
