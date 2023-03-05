use std::collections::HashMap;
use regex::Regex;

#[path = "../../types/Lexer.rs"] mod lex;

pub fn matchesChar(input: char)-> bool {
  let re = Regex::new(r"\s]");
  return re.is_match(input);
}

pub const WhiteSpaceTokenizer: lex::Tokenizer = lex::Tokenizer {
  token_type: "whitespace".to_string(),
  matchesChar: matchesChar,
  handlChar: |
    initial_char: char,
    advance: fn()->char,
    addToken: fn(values: HashMap<String, String>)->(),
  |->Result<(), String>{
    advance();
    return Ok(());
  },
};

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