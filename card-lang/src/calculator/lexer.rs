use std::io::ErrorKind;

use regex::Regex;

// use crate::types::Lexer;

use crate::types::Lexer;

// #[path = "../types/Lexer.rs"] mod Lexer;
#[path = "./tokens/digit.rs"] mod digit;
#[path = "./tokens/identifier.rs"] mod id;
#[path = "./tokens/operator.rs"] mod op;
#[path = "./tokens/whitespace.rs"] mod ws;


// https://www.codeproject.com/Articles/345888/How-to-Write-a-Simple-Interpreter-in-JavaScript

pub struct CalculatorLexer {}

impl Lexer::Lexer for CalculatorLexer {

  fn getTokenizers() -> Vec<Lexer::Tokenizer> {
    return vec![
      ws::WhiteSpaceTokenizer,
      op::OperatorTokenizer,
      digit::DigitTokenizer,
      id::IdentifierTokenizer
    ];
  }
}

impl CalculatorLexer {
  fn isNumberPrefix(input: char)->bool{
    return input == '🔢';
  }
}
