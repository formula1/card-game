
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

use calculator::lexer::createCalculatorLexer;
use calculator::parser::createCalculatorParser;
use calculator::evaluator::createCalculatorEvaluator;

fn main() {

  let lexer = createCalculatorLexer();
  let parser = createCalculatorParser();
  let evaluator = createCalculatorEvaluator();

  let input_str = "hello world";
  let tokens = lexer.tokenizeString(input_str.to_string());
  let trees = parser.parse(tokens);
  let output = evaluator.evaluate(trees);
  println!("{}", output.as_str());
}
