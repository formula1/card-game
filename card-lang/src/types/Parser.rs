

use std::collections::HashMap;

use crate::types::ReusedStructs::Token;
use crate::types::ReusedStructs::Node;
use crate::types::ReusedStructs::NodeType;

pub struct Parser {
  symbols: SymbolCollection,
  i: usize,
  tokens: Vec<Token>,
}

pub struct Symbol {
  id: String,
  nud: Option<Box<dyn NudListener>>,
  lbp: Option<u128>,
  led: Option<Box<dyn LedListener>>,
}

pub trait NudListener {
  fn run(self, symtok: SymbolAndToken, parser: Parser)->Node;
}

pub trait LedListener {
  fn run(self, node: Node, parser: Parser)->Node;
}

pub struct Prefix {
  id: String,
  rbp: u128
}

pub struct Infix {
  id: String,
  lbp: u128,
  rbp: Option<u128>,
  led: Option<Box<dyn LedListener>>
}

impl Parser {
  pub fn new(prefixes: Vec<Prefix>, infixes: Vec<Infix>, symbols: Vec<Symbol>) -> Parser {
    let parser = Parser {
      symbols: SymbolCollection { symbols: HashMap::new() },
      i: 0,
      tokens: vec![]
    };
    for prefix in prefixes {
      parser.prefix(prefix.id.as_str(), prefix.rbp);
    }
    for infix in infixes {
      parser.infix(infix.id.as_str(), infix.lbp, infix.rbp, infix.led);
    }
    for symbol in symbols {
      parser.symbol(symbol.id.as_str(), symbol.nud, symbol.lbp, symbol.led);
    }
    return parser;
  }
  pub fn token(self) -> (Self, SymbolAndToken) {
    return (
      self,
      self.symbols.interpretToken(self.tokens[self.i].clone())
    );
  }
  pub fn nextToken(self) -> (Self, Token) {
    return (
      self,
      self.tokens[self.i + 1].clone()
    );
  }
  pub fn advance(mut self) -> (Self, SymbolAndToken){
    self.i += 1;
    return self.token()
  }

  pub fn expression(self, rbp: u128) -> (Self, Node) {
    let mut s: Self = self;
    let t1: SymbolAndToken;
    (s, t1) = s.token();
    (s, _) = s.advance();
    let nud = t1.symbol.nud;
    if nud.is_none() {
      panic!("Unexpected token: {}", t1.token.token_type.as_str());
    }
    let mut left = nud.unwrap().run(t1, s);
    loop {
      let t: SymbolAndToken;
      (s, t) = s.token();
      if rbp < t.symbol.lbp.unwrap() { break; }
      (s, _) = s.token();
      (s, _) = s.advance();
      let led_op = t.symbol.led;
      if led_op.is_none() {
        panic!("Unexpected token: {}", t.token.token_type.as_str());
      }
      let led = led_op.unwrap();

      left = led.run(left, s);
    };
    return (s, left);
  }

  pub fn parse(mut self, tokens: Vec<Token>)->Vec<Node>{
    self.tokens = tokens;
    let mut parseTree = vec![];
    let mut s = self;
    let mut n: Node;
    loop {
      let t:SymbolAndToken;
      (s, t) = s.token();
      if t.token.token_type == "(end)" { break }
      (s, n) = s.expression(0);
      parseTree.push(n);
    }

    return parseTree;
  }

  pub fn symbol(self, id: &str, nud: Option<Box<dyn NudListener>>, lbp: Option<u128>, led: Option<Box<dyn LedListener>>){
    self.symbols.setSymbol(id, nud, lbp, led);
  }

  pub fn infix(self, id: &str, lbp: u128, rbp_op: Option<u128>, led_maybe: Option<Box<dyn LedListener>>) {
    let rbp = if rbp_op != None { rbp_op.unwrap() } else { lbp };
    let led = if !led_maybe.is_none() { led_maybe.unwrap() } else {
      Box::new(DefaultInfix {
        rbp: rbp,
        parser: self,
        symbol_id: id.to_string()
      })
    };
    self.symbols.setSymbol(id, None, Some(lbp), Some(led));
  }
  pub fn prefix(self, id: &str, rbp: u128) {
    let pre = Box::new(DefaultPrefix {
      parser: self,
      symbol_id: id.to_string(),
      rbp: rbp
    });
    self.symbols.setSymbol(id, Some(pre), None, None);
  }
}


pub struct SymbolCollection {
  symbols: HashMap<String, Symbol>
}


pub struct SymbolAndToken {
  pub symbol: Symbol,
  pub token: Token
}

impl SymbolCollection {
  fn setSymbol(mut self, id_str: &str, nud: Option<Box<dyn NudListener>>, lbp: Option<u128>, led: Option<Box<dyn LedListener>>){
    let id = id_str.to_string();
    let maybe_sym = self.symbols.get(&id);
    if maybe_sym.is_none() {
      self.symbols.insert(id.clone(), Symbol {
        id, nud, lbp, led
      });
    } else {
      let sym = maybe_sym.unwrap();
      sym.nud = if !sym.nud.is_none() { sym.nud } else { nud };
      sym.lbp = if !sym.lbp.is_none() { sym.lbp } else { lbp };
      sym.led = if !sym.led.is_none() { sym.led } else { led };
    }
  }
  fn interpretToken(self, token: Token) -> SymbolAndToken {
    let token_type = if token.token_type == "operator" {
      token.values.get("value").unwrap().clone()
    } else {
      token.token_type.clone()
    };
    let sym = self.symbols.get(&token_type).unwrap();
    return SymbolAndToken{
      token: token,
      symbol: *sym
    }
  }
}

struct DefaultPrefix {
  parser: Parser,
  symbol_id: String,
  rbp: u128
}

impl NudListener for DefaultPrefix {
  fn run(&self, symtok: SymbolAndToken, parser: Parser)->Node{

    let values = HashMap::from([("type".to_string(), self.symbol_id.clone())]);
    let (s, n) = parser.expression(self.rbp);
    let branches = HashMap::from([
      ("right".to_string(), n),
    ]);


    return Node {
      node_type: NodeType::OperatorNode,
      values: Some(values),
      branches: Some(branches),
      args: None,
    }
  }

}

struct DefaultInfix {
  rbp: u128,
  parser: Parser,
  symbol_id: String
}

impl LedListener for DefaultInfix {
  fn run(&self, left: Node, parser: Parser)->Node{
  
    let values = HashMap::from([("type".to_string(), self.symbol_id.clone())]);
    let (s, n ) = parser.expression(self.rbp);
    let branches = HashMap::from([
      ("left".to_string(), left),
      ("right".to_string(), n),
    ]);
  
    return Node {
      node_type: NodeType::OperatorNode,
      values: Some(values),
      branches: Some(branches),
      args: None,
    }  
  }
  
}

