use std::collections::HashMap;
use regex::Regex;

use crate::types::Lexer;

pub const OperatorTokenizer: Lexer::Tokenizer = Lexer::Tokenizer {
  token_type: "operator".to_string(),
  matchesChar: matchesChar,
  handleChar: handleChar
};

const op_regex: Regex = match Regex::new(r"[+\-*\/\^%=(),]") {
  Err(e) => panic!("Bad regex in operator tokenizer"),
  Ok(regex) => regex
};

pub fn matchesChar(input: char) -> bool{
  return op_regex.is_match(input.to_string().as_str());
}

fn handleChar(
  c: char,
  advance: fn()->char,
  addToken: fn(values: HashMap<String, String>)->(),
)-> Result<(), String> {
  addToken(
    HashMap::from([("value".to_string(), c.to_string())])
  );
  advance();
  return Ok(());
}

