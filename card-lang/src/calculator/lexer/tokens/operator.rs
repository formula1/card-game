use std::collections::HashMap;
use regex::Regex;

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


const op_regex: Regex = match Regex::new(r"[+\-*\/\^%=(),]") {
  Err(e) => panic!("Bad regex in operator tokenizer"),
  Ok(regex) => regex
};

pub fn matchesChar(input: char) -> bool{
  return op_regex.is_match(input.to_string().as_str());
}

fn handleChar(c: char, lexer: Lexer)-> Result<(), String> {
  lexer.addToken(
    HashMap::from([("value".to_string(), c.to_string())])
  );
  lexer.advance();
  return Ok(());
}

