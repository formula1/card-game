use std::io::ErrorKind;

use regex::Regex;

// use crate::types::Lexer;

use crate::types::Lexer::Lexer;

// #[path = "../types/Lexer.rs"] mod Lexer;
#[path = "./tokens/digit.rs"] mod digit;
#[path = "./tokens/identifier.rs"] mod id;
#[path = "./tokens/operator.rs"] mod op;
#[path = "./tokens/whitespace.rs"] mod ws;


// https://www.codeproject.com/Articles/345888/How-to-Write-a-Simple-Interpreter-in-JavaScript


pub fn createCalculatorLexer<'a>() -> Lexer<'a> {
  return Lexer::new(
    vec![
      Box::new(ws::WhiteSpaceTokenizer{}),
      Box::new(op::OperatorTokenizer{}),
      Box::new(digit::DigitTokenizer{}),
      Box::new(id::IdentifierTokenizer{})
    ]
  );
}

// pub static CalculatorLexer: Lexer::Lexer = Lexer::Lexer::new(
//   vec![
//     Box::new(ws::WhiteSpaceTokenizer{}),
//     Box::new(op::OperatorTokenizer{}),
//     Box::new(digit::DigitTokenizer{}),
//     Box::new(id::IdentifierTokenizer{})
//   ]
// );

// impl CalculatorLexer {
//   fn isNumberPrefix(input: char)->bool{
//     return input == '🔢';
//   }
// }
