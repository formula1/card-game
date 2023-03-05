use std::collections::HashMap;
use regex::Regex;

#[path = "../../types/Lexer.rs"] mod lex;
#[path = "./operator.rs"] mod op;
#[path = "./digit.rs"] mod digit;
#[path = "./whitespace.rs"] mod ws;

pub fn matchesChar(input: char){
  return (
    !op::matchesChar(input)
    && !digit::matchesChar(input)
    && !ws::matchesChar(input)
  )
}

pub const IdentifierTokenizer: lex::Tokenizer = lex::Tokenizer {
  token_type: "identifier".to_string(),
  matchesChar: matchesChar,
  handlChar: |
    initial_char: char,
    advance: fn()->char,
    addToken: fn(values: HashMap<String, String>)->(),
  |->Result<(), String>{
    let identity = "".to_string();
    identity += initial_char;
    loop {
      initial_char = advance();
      if !matchesChar(initial_char) {
        break;
      }
      identity += initial_char;
    }
    addToken(HashMap::from([("name", identity.to_owned())]));
    advance();
    return Ok(0);
  },
};
