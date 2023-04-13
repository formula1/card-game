
use core::panic;
use std::{collections::HashMap, str::Chars};

use crate::types::ReusedStructs::Token;


pub struct Lexer<'a> {
  tokenizers: Vec<Box<dyn Tokenizer>>,
  current_tokenizer: String,
  input_chars: Chars<'a>,
  current_char: Option<char>,
  tokens: Vec<Token>
}

impl Lexer<'_> {
  pub fn new(tokenizers: Vec<Box<dyn Tokenizer>>)-> Lexer<'static> {
    return Lexer {
      tokenizers: tokenizers,
      current_tokenizer: "".to_string(),
      input_chars: "".chars(),
      current_char: None,
      tokens: vec![]
    };
  }
  fn reset(&self)->(){
    self.current_tokenizer = "".to_string();
    self.input_chars = "".chars();
    self.current_char = None;
    self.tokens = vec![];
  }
  pub fn advance(&self)->char{
    let c = self.input_chars.next();
    self.current_char = c;
    return c.unwrap();
  }
  pub fn addToken(&self, values: HashMap<String, String>)->(){
    self.tokens.push(Token {
      token_type: self.current_tokenizer.clone(),
      values: values
    });
  }
  pub fn tokenizeString(&self, input_str: String) -> Vec<Token>{
    let mut s = self.reset();
    let tokenizers = &self.tokenizers;
    self.input_chars = input_str.chars();
    self.advance();


    while self.current_char != None {
      let mut usedTokenizer = false;
      for t in tokenizers {
        if !t.matchesChar(self.current_char.clone().unwrap()) {
          continue;
        }
        usedTokenizer = true;
        self.current_tokenizer = t.token_type();
        if let Err(e) = t.handleChar(self.current_char.clone().unwrap(), self) {
          panic!("Error parsing the code");
        }
      }
      if usedTokenizer == false {
        panic!("invalid token {}", self.current_char.unwrap());
      }
    }
    return self.tokens;
  }

}

pub trait Tokenizer {
  fn token_type(&self)-> String;
  fn matchesChar(&self, c: char)->bool;
  fn handleChar(
    &self, c: char, lexer: &Lexer
  )->Result<(), String>;
}

// pub struct Tokenizer<Adv, Tok>{
//   token_type: String,
//   matchesChar: fn(c: char)->bool,
//   handleChar: fn(
//     initial_char: char,
//     advance: Adv,
//     addToken: Tok,
//   )->Result<(), String> where Adv: Fn()->char, Tok: Fn(HashMap<String, String>)->()
// }

// advance: impl Fn()->char,
// addToken: fn(values: HashMap<String, String>)->(),

