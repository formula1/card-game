use std::collections::HashMap;
use regex::Regex;

use crate::types::Lexer::Lexer;
use crate::types::Lexer::Tokenizer;

pub struct DigitTokenizer{}

impl Tokenizer for DigitTokenizer {
  fn token_type(self)->String{
    return "digit".to_string();
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


const digit_regex: Regex = match Regex::new(r"[0-9]") {
  Err(e) => panic!("Bad regex in digit tokenizer"),
  Ok(regex) => regex
};

pub fn matchesChar(input: char) -> bool{
  return digit_regex.is_match(input.to_string().as_str());
}

fn handleChar(c: char, lexer: Lexer) -> Result<(), String>{
  let mut num = "".to_owned();
  num.push(c);
  let isFloat = "0";
  loop {
    c = lexer.advance();
    if !matchesChar(c) { break; }
    num.push(c);
  }
  if c == '.' {
    isFloat = "1";
    loop {
      num.push(c);
      c = lexer.advance();
      if !matchesChar(c) { break; }
    }
  }
  lexer.addToken(
    HashMap::from([
      ("isFloat".to_string(), isFloat.to_string()),
      ("value".to_string(), num)
    ])
  );
  return Ok(());
}


