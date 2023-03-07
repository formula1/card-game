
mod calculator {
    pub mod lexer;
    pub mod parser;
}
mod types {
    pub mod Lexer;
    pub mod Parser;
    pub mod Evaluator;
    pub mod ReusedStructs;
}

use calculator::lexer::CalculatorLexer;

fn main() {
    let tokens = CalculatorLexer.tokenizeString(
        "hello world".to_string()
    );
    let parser = calculator::parser::CalculatorParser::new(tokens);
    let parsed = 
    println!("Hello, world!");
}
