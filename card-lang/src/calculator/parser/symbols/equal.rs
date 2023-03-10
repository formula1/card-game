use std::collections::HashMap;

use crate::types::Parser::Parser;
use crate::types::Parser::LedListener;
use crate::types::Parser::SymbolAndToken;
use crate::types::ReusedStructs::Node;
use crate::types::ReusedStructs::NodeType;
use crate::types::ReusedStructs::Token;


pub struct EqualLed{}

impl LedListener for EqualLed {
  fn run(self, left: Node, parser: Parser) -> Node {
    let name = left.values.unwrap().get("name").unwrap();
    let node = Node {
      node_type: NodeType::AssignNode,
      values: Some(HashMap::from([
        ("name".to_string(), *name),
      ])),
      branches: Some(HashMap::from([
        ("value".to_string(), parser.expression(2)),
      ])),
      args: None
    };
    let node_type = left.node_type;
    match node_type {
      NodeType::CallNode => {
        if left.args.is_none() {
          panic!("there should be args when making a call");
        }
        let args = left.args.unwrap();
        for arg in args {
          match arg.node_type {
            NodeType::IdentifierNode => {}
            _ => {
              panic!("invalid argument name");
            }
          }
        }
        node.args = Some(args);
      }
      NodeType::IdentifierNode => {}
      _ => {
        panic!("Invalid node type");
      }
    }
    return node
  }
}