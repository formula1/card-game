use std::io::ErrorKind;

use regex::Regex;

#[path = "../types/Lexer.rs"] mod lexer;
#[path = "./tokens/digit.rs"] mod digit;
#[path = "./tokens/identifier.rs"] mod id;
#[path = "./tokens/operator.rs"] mod op;
#[path = "./tokens/whitespace.rs"] mod ws;


// https://www.codeproject.com/Articles/345888/How-to-Write-a-Simple-Interpreter-in-JavaScript

pub struct CalculatorLexer {}


impl lexer::Lexer for CalculatorLexer {

  fn getTokenizers() -> Vec<lexer::Tokenizer> {
      return vec![
        ws::WhiteSpaceTokenizer{},
        op::OperatorTokenizer{},
        digit::DigitTokenizer{},
        id::IdentifierTokenizer{}
      ];
  }
}

impl CalculatorLexer {
  fn isNumberPrefix(input: char)->bool{
    return input == "🔢";
  }
}
