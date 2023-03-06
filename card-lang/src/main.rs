
mod calculator {
    pub mod lexer;
}
mod types {
    pub mod Lexer;
}

fn main() {
    calculator::lexer::CalculatorLexer::tokenizeString("hello world");
    println!("Hello, world!");
}
