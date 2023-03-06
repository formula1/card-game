use std::collections::HashMap;
use regex::Regex;

use crate::types::Lexer;

struct OperatorTokenizer{}

pub const DigitTokenizer: Lexer::Tokenizer = Lexer::Tokenizer {
  token_type: "digit".to_string(),
  matchesChar: matchesChar,
  handleChar: handleChar,
};

const digit_regex: Regex = match Regex::new(r"[0-9]") {
  Err(e) => panic!("Bad regex in digit tokenizer"),
  Ok(regex) => regex
};

pub fn matchesChar(input: char) -> bool{
  return digit_regex.is_match(input.to_string().as_str());
}

fn handleChar(
  c: char,
  advance: fn()->char,
  addToken: fn(values: HashMap<String, String>)->(),
) -> Result<(), String>{
  let mut num = "".to_owned();
  num.push(c);
  let isFloat = "0";
  loop {
    c = advance();
    if !matchesChar(c) { break; }
    num.push(c);
  }
  if c == '.' {
    isFloat = "1";
    loop {
      num.push(c);
      c = advance();
      if !matchesChar(c) { break; }
    }
  }
  addToken(
    HashMap::from([
      ("isFloat".to_string(), isFloat.to_string()),
      ("value".to_string(), num)
    ])
  );
  return Ok(());
}


