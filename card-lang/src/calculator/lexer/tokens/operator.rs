use std::collections::HashMap;
use lazy_regex::regex_is_match;

use crate::types::Lexer::Lexer;
use crate::types::Lexer::Tokenizer;

pub struct OperatorTokenizer{}

impl Tokenizer for OperatorTokenizer {
  fn token_type(self)->String{
    return "operator".to_string();
  }
  fn matchesChar(self, input: char)->bool{
    return matchesChar(input)
  }
  fn handleChar(
    self, c: char, lexer: Lexer
  )->Result<(), String>{
    return handleChar(c, lexer);
  }
}


pub fn matchesChar(input: char) -> bool{
  return regex_is_match!(r"[+\-*/^%=(),]", input.to_string().as_str());
}

fn handleChar(c: char, mut lexer: Lexer)-> Result<(), String> {
  lexer.addToken(
    HashMap::from([("value".to_string(), c.to_string())])
  );
  lexer.advance();
  return Ok(());
}

