use std::collections::HashMap;
use regex::Regex;

#[path = "../../types/Lexer.rs"] mod lex;


pub fn matchesChar(input: char)->bool{
  let re = Regex::new(r"[+\-*\/\^%=(),]");
  return re.is_match(input.to_string());
}

pub const OperatorTokenizer: lex::Tokenizer = lex::Tokenizer {
  token_type: "operator".to_string(),
  matchesChar: matchesChar,
  handlChar: |
    initial_char: char,
    advance: fn()->char,
    addToken: fn(values: HashMap<String, String>)->(),
  |->Result<(), String>{
    addToken(
      HashMap::from([("value", initial_char.to_string())])
    );
    advance();
    return Ok(0);
  },
};
