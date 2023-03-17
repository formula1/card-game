use std::collections::HashMap;

use crate::types::Parser::Parser;
use crate::types::Parser::NudListener;
use crate::types::Parser::SymbolAndToken;
use crate::types::ReusedStructs::Node;
use crate::types::ReusedStructs::NodeType;

pub struct NumberNud {}

impl NudListener for NumberNud {
  fn run(&self, symtok: SymbolAndToken, _: Parser)->Node{
    return Node {
      node_type: NodeType::ValueNode,
      values: Some(HashMap::from([
        ("isFLoat".to_string(), symtok.token.values.get("isFloat").unwrap().clone()),
        ("value".to_string(), symtok.token.values.get("value").unwrap().clone()),
      ])),
      branches: None,
      args: None
    };
  }
}
