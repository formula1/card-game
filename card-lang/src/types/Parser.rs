

use std::collections::HashMap;

use crate::types::Lexer;
use crate::types::ReusedStructs::Token;
use crate::types::ReusedStructs::Node;

pub struct Parser {
  symbols: SymbolCollection,
  i: usize,
  tokens: Vec<Token>,
}

impl Parser {
  fn new(tokens: Vec<Token>) -> Parser {
    return Parser {
      symbols: SymbolCollection { symbols: HashMap::new() },
      i: 0,
      tokens: vec![]
    }
  }
  fn token(self) -> SymbolAndToken {
    return self.symbols.interpretToken(self.tokens[self.i]);
  }
  fn advance(&mut self) -> SymbolAndToken{
    self.i += 1;
    return self.token();
  }

  fn expression(self, rbp: u128) -> Node {
    let left = None;
    let t = self.token();
    self.advance();
    if t.nud == None {
      panic!("Unexpected token: {}", t.token_type.as_str());
    }
    left = t.nud(t);
    while rbp < self.token().lbp {
      t = self.token();
      self.advance();
      if t.led == None {
        panic!("Unexpected token: {}", t.token_type.as_str());
      }
      left = t.led(left);
    };
    return Ok(left);
  }

  fn parse(&mut self, tokens: Vec<Token>){
    self.tokens = tokens;
    let parseTree = vec![];

    while self.token().token_type != "(end)" {
      parseTree.push(self.expression(0));
    }

    return parseTree;
  }

  pub fn symbol(&mut self, id: String, nud: Option<fn(HashMap<String, String>)->()>, lbp: Option<u128>, led: Option<String>){
    self.symbols.setSymbol(id, nud, lbp, led);
  }

  pub fn infix(self, id: String, lbp: u128, rbp: Option<u128>, led_maybe: Option<impl Fn(Option<Node>)->Node>) {
    let rbp_val = if rbp != None { rbp.unwrap() } else { lbp };
    let led = if led_maybe != None { led_maybe.unwrap() } else { |left_maybe: Option<Node>| -> Node {
      return defaultInfix(id, self, rbp, left_maybe);
    }};
    self.symbols.setSymbol(id, None, Some(lbp), led);
  }
  pub fn prefix(self, id: String, rbp: u128) {
    self.symbols.setSymbol(id, || {
      return Node::
      return Branch {
        branch_type: id,
        left: None,
        right: self.expression(rbp)
      };
    }, None, None);
  }
}


pub struct SymbolCollection {
  symbols: HashMap<String, Symbol>
}

pub trait Symbol {
  fn own_parser(self)->Parser;
  fn get_id(self)->String;
  fn nud(self)->Node;
  fn lbp(self)->u128;
  fn led(self, node: Option<Node>)->Node;
}

// struct Symbol {
//   owner: SymbolCollection,
//   id: String,
//   nud: Option<fn(SymbolAndToken)->()>,
//   lbp: Option<u128>,
//   led: Option<fn(Option<Node>)->Node>,
// }

struct SymbolAndToken {
  symbol: Symbol,
  token: Token
}

impl SymbolCollection {
  fn setSymbol(&mut self, id: String, nud: Option<fn(HashMap<String, String>)->()>, lbp: Option<u128>, led: Option<impl Fn(Option<Node>)->Node>){
    let maybe_sym = self.symbols.get(id);
    if maybe_sym == None {
      self.symbols.set(id, Symbol {
        owner: self,
        id, nud, lbp, led
      })
    } else {
      let sym = maybe_sym.unwrap();
      sym.nud = if sym.nud != None { sym.nud } else { nud };
      sym.lbp = if sym.lbp != None { sym.lbp } else { lbp };
      sym.led = if sym.led != None { sym.led } else { led };
    }
  }
  fn interpretToken(self, token: Token) -> SymbolAndToken {
    let maybe_sym = self.symbols.get(token.token_type);
    if maybe_sym == None {
      panic!("Token has no symbol {}", token.token_type());
    }
    let sym = maybe_sym.unwrap();
    return SymbolAndToken{
      token: token,
      symbol: sym
    }
  }
}

pub fn defaultInfix(symbol: Symbol, parser: Parser, rbp: u128, left_maybe: Option<Node>){
  if left_maybe == None {
    panic!("Expected a left side")
  }
  let left = left_maybe.unwrap();

  let values = HashMap::from([("type".to_string(), symbol.id)]);
  let branches = HashMap::from([
    ("left".to_string(), left),
    ("right".to_string(), parser.expression(rbp)),
  ]);

  return Node {
    node_type: "operator".to_string(),
    values: Some(values),
    branches: Some(branches)
  }  
}

