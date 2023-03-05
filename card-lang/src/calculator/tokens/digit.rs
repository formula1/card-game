use std::collections::HashMap;
use regex::Regex;

#[path = "../../types/Lexer.rs"] mod lex;


pub fn matchesChar(input: char){
  let re = Regex::new(r"[0-9]");
  return re.is_match(input);
}

pub const OperatorTokenizer: lex::Tokenizer = lex::Tokenizer {
  token_type: "digit".to_string(),
  matchesChar: matchesChar,
  handlChar: |
    c: char,
    advance: fn()->char,
    addToken: fn(values: HashMap<String, String>)->(),
  |->Result<(), String>{
    let num = "";
    num += c;
    let isFloat = "0";
    loop {
      c = advance();
      if !matchesChar(c) { break; }
      num += c;
    }
    if c == "." {
      isFloat = "1";
      loop {
        num += c;
        c = advance();
        if !matchesChar(c) { break; }
      }
    }
    addToken(
      HashMap::from([
        ("isFloat", isFloat),
        ("value", num)
      ])
    );
    return Ok(0);
  },
};

