use std::collections::HashMap;
use lazy_regex::regex_is_match;

use crate::types::Lexer::Lexer;
use crate::types::Lexer::Tokenizer;

pub struct DigitTokenizer{}

impl Tokenizer for DigitTokenizer {
  fn token_type(&self)->String{
    return "digit".to_string();
  }
  fn matchesChar(&self, input: char)->bool{
    return matchesChar(input)
  }
  fn handleChar(
    &self, c: char, lexer: &Lexer
  )->Result<(), String>{
    return handleChar(c, lexer);
  }
}


pub fn matchesChar(input: char) -> bool{
  return regex_is_match!(r"[0-9]", input.to_string().as_str());
}

fn handleChar(initial_char: char, mut lexer: &Lexer) -> Result<(), String>{
  let mut c: char = initial_char;
  let mut l = lexer;
  let mut num = "".to_owned();
  num.push(c);
  let mut is_float = "0";
  loop {
    c = l.advance();
    if !matchesChar(c) { break; }
    num.push(c);
  }
  if c == '.' {
    is_float = "1";
    loop {
      num.push(c);
      c = l.advance();
      if !matchesChar(c) { break; }
    }
  }
  l.addToken(
    HashMap::from([
      ("is_float".to_string(), is_float.to_string()),
      ("value".to_string(), num)
    ])
  );
  return Ok(());
}


