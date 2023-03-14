use std::collections::HashMap;

use crate::types::Parser::Parser;
use crate::types::Parser::NudListener;
use crate::types::Parser::SymbolAndToken;
use crate::types::ReusedStructs::Node;
use crate::types::ReusedStructs::NodeType;
use crate::types::ReusedStructs::Token;


pub struct IdentifierNud {}

impl NudListener for IdentifierNud {
  fn run(self, symtok: SymbolAndToken, parser: Parser)->Node{
    let token = symtok.token.clone();
    if !isOpener(symtok.token) {
      return Node {
        node_type: NodeType::IdentifierNode,
        values: Some(token.values),
        branches: None,
        args: None
      }
    }

    let mut args: Vec<Node> = vec![];
    if isCloser(parser.nextToken()) {
      parser.advance();
    } else {
      loop {
        parser.advance();
        args.push(parser.expression(2));
        if!isComma(parser.token().token) { break }
      }
      if !isCloser(parser.token().token) {
        panic!("Expected closing parenthesis ')'");
      };
    }
    parser.advance();
    return Node {
      node_type: NodeType::CallNode,
      values: Some(token.values),
      args: Some(args),
      branches: None,
    }
  }
}

fn isOpener(token: Token) -> bool {
  if token.token_type != "operator" {
    return false
  }
  let str: String = token.values.get("value").unwrap().clone();
  if  str != "(".to_string() {
    return false;
  }
  return true;
}

fn isCloser(token: Token) -> bool {
  if token.token_type != "operator" {
    return false
  }
  let str = token.values.get("value").unwrap().clone();
  if  str != ")".to_string() {
    return false;
  }
  return true;
}

fn isComma(token: Token) -> bool {
  if token.token_type != "operator" {
    return false
  }
  let str = token.values.get("value").unwrap().clone();
  if  str != ",".to_string() {
    return false;
  }
  return true;

}
