
mod calculator {
  pub mod lexer;
  pub mod parser;
  pub mod evaluator;
}

mod types {
  pub mod Lexer;
  pub mod Parser;
  pub mod Evaluator;
  pub mod ReusedStructs;
}

use calculator::lexer::CalculatorLexer;
use calculator::parser::CalculatorParser;

fn main() {
  let tokens = CalculatorLexer.tokenizeString(
    "hello world".to_string()
  );
  let trees = CalculatorParser.parse(tokens);
  println!("Hello, world!");
}
