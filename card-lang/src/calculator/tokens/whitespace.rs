use std::collections::HashMap;
use regex::Regex;

use crate::types::Lexer::Lexer;
use crate::types::Lexer::Tokenizer;

pub struct WhiteSpaceTokenizer{}

impl Tokenizer for WhiteSpaceTokenizer {
  fn token_type(self)->String{
    return "whitespace".to_string();
  }
  fn matchesChar(self, input: char)->bool{
    return matchesChar(input)
  }
  fn handleChar(
    self, c: char, lexer: Lexer
  )->Result<(), String>{
    return handleChar(c, lexer);
  }
}

const ws_regex: Regex = match Regex::new(r"\s]") {
  Err(e) => panic!("Bad regex in whitespace tokenizer"),
  Ok(regex) => regex
};

pub fn matchesChar(input: char) -> bool{
  return ws_regex.is_match(input.to_string().as_str());
}

fn handleChar(initial_char: char, lexer: Lexer) ->Result<(), String> {
  lexer.advance();
  return Ok(());
}


// impl lex::Tokenizer for WhiteSpaceTokenizer {
//   fn token_typeStatic()->String {
//       return "whitespace".to_string()
//   }
//   fn matchesTypeStatic(self, input: char){
//     let re = Regex::new(r"\s]");
//     return re.is_match(input);
//   }
//   fn handleChar(
//     self,
//     initial_char: char,
//     advance: fn()->char,
//     addToken: fn(value: HashMap<String, String>)->(),
//   ){
//     advance();
//   }

// }