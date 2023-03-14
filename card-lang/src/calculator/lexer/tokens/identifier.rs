use std::collections::HashMap;
use regex::Regex;
use lazy_regex::regex_is_match;

use crate::types::Lexer::Lexer;
use crate::types::Lexer::Tokenizer;
#[path = "./operator.rs"] mod op;
#[path = "./digit.rs"] mod digit;
#[path = "./whitespace.rs"] mod ws;

pub struct IdentifierTokenizer{}

impl Tokenizer for IdentifierTokenizer {
  fn token_type(&self)->String{
    return "identifier".to_string();
  }
  fn matchesChar(&self, input: char)->bool{
    return matchesChar(input)
  }
  fn handleChar(
    &self, c: char, lexer: Lexer
  )->Result<(), String>{
    return handleChar(c, lexer);
  }
}

pub fn matchesChar(input: char) -> bool{
  return !op::matchesChar(input) && !digit::matchesChar(input) && !ws::matchesChar(input);
}

fn handleChar(initial_char: char, mut lexer: Lexer) -> Result<(), String> {
  let mut c = initial_char;
  let mut identity = "".to_string();
  identity.push(c);
  loop {
    c = lexer.advance();
    if !matchesChar(c) {
      break;
    }
    identity.push(c);
  }
  lexer.addToken(HashMap::from([("name".to_string(), identity.to_owned())]));
  lexer.advance();
  return Ok(());
}