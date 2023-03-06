
use core::panic;
use std::collections::HashMap;

pub trait Lexer {
  fn getTokenizers() -> Vec<Tokenizer>;
  fn tokenizeString(input_str: String) -> Result<Vec<Token>, String>{
    let tokenizers = Self::getTokenizers();
    let input_chars = input_str.chars();
    let tokens: Vec<Token> = vec![];
    let mut c = input_chars.next();
    let advance = || {
      c = input_chars.next();
      if c == None {
        panic!("No more characters")
      }
      return c.unwrap();
    };
    let token_type = "".to_string();
    let addToken = |values: HashMap<String, String>| -> () {
      tokens.push(Token {
        token_type: token_type.to_string(),
        values: values
      })
    };
    while c != None {
      let usedTokenizer = false;
      for t in tokenizers {
        if !(t.matchesChar)(c.unwrap()) {
          continue;
        }
        usedTokenizer = true;
        token_type = t.token_type;
        if let Err(e) = (t.handleChar)(c.unwrap(), advance, addToken) {
          println!("Error parsing the code");
          return Err(e);
        }
      }
      if usedTokenizer == false {
        return Err(
          format!("invalid token {} at point {}", c.unwrap(), "")
        );
      }
    }
    return Ok(tokens);
  }

}

pub struct Tokenizer{
  token_type: String,
  matchesChar: fn(c: char)->bool,
  handleChar: fn(
    initial_char: char,
    advance: fn()->char,
    addToken: fn(values: HashMap<String, String>)->(),
  )->Result<(), String>
}

struct Token {
  token_type: String,
  values: HashMap<String, String>,
}