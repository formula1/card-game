
use std::collections::HashMap;



pub trait Lexer {
  fn getTokenizers() -> Vec<Tokenizer>;
  fn tokenizeString(input_str: String){
    let tokenizers = Self::getTokenizers();
    let input_chars = input_str.chars();
    let tokens: Vec<Token> = vec![];
    let i: usize = 0;
    let c: char = input_chars.nth(i).unwrap();
    let advance = || {
      i += 1;
      c = input_chars.nth(i);
      return c;
    };
    let token_type = "";
    let addToken = |values: HashMap<String, String>| -> () {
      tokens.push(Token { token_type: token_type, values: values })
    };
    input_str.chars()[i];
    while i < input_chars.length() {
      let usedTokenizer = false;
      for t in tokenizers {
        if !t.matchesType(c) {
          continue;
        }
        usedTokenizer = true;
        token_type = t.token_type();
        let r = t.handleChar(c, i, advance, addToken);
        match r {
          Err(e) => {
            println!("Error parsing the code");
            return r
          }
        }
      }
      if usedTokenizer == false {
        let errmsg = "invalid token " + c + " at point " + i;
        return Err(errmsg);
      }
    }
    return Ok(tokens);
  }

}

pub struct Tokenizer{
  token_type: String,
  matchesChar: fn(c: char)->bool,
  handlChar: fn(
    initial_char: char,
    advance: fn()->char,
    addToken: fn(values: HashMap<String, String>)->(),
  )->Result<(), String>
}

struct Token {
  token_type: String,
  values: HashMap<String, String>,
}