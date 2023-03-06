use std::collections::HashMap;
use regex::Regex;

use crate::types::Lexer;
#[path = "./operator.rs"] mod op;
#[path = "./digit.rs"] mod digit;
#[path = "./whitespace.rs"] mod ws;


pub const IdentifierTokenizer: Lexer::Tokenizer = Lexer::Tokenizer {
  token_type: "identifier".to_string(),
  matchesChar: matchesChar,
  handleChar: handleChar,
};

pub fn matchesChar(input: char) -> bool{
  return !op::matchesChar(input) && !digit::matchesChar(input) && !ws::matchesChar(input);
}

fn handleChar(
  c: char,
  advance: fn()->char,
  addToken: fn(values: HashMap<String, String>)->(),
) -> Result<(), String> {
  let identity = "".to_string();
  identity.push(c);
  loop {
    c = advance();
    if !matchesChar(c) {
      break;
    }
    identity.push(c);
  }
  addToken(HashMap::from([("name".to_string(), identity.to_owned())]));
  advance();
  return Ok(());
}