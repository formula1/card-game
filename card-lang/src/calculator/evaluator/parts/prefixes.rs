use std::collections::HashMap;

use crate::types::Evaluator::PrefixOperator;
use crate::types::Evaluator::PrefixRunner;
use crate::types::ReusedStructs::Node;
use crate::types::ReusedStructs::NodeType;


pub const prefixes: Vec<PrefixOperator> = vec![
  PrefixOperator{ id: "-".to_string(), runner: Box::new(SubRunner{}) },
];

struct SubRunner{}
impl PrefixRunner for SubRunner {
  fn run(self, a: Node)-> Node {
    let a_num = extractNumber(a);
    return packResult(-1_f64 * a_num);
  }
}


pub fn extractNumber(a: Node) -> f64 {
  if !matches!(a.node_type, NodeType::ValueNode) {
    panic!("a node_type is not ValueNode");
  }
  return a.values.unwrap()["value"].parse::<f64>().unwrap();
}

pub fn packResult(num: f64)->Node{
  Node {
    node_type: NodeType::ValueNode,
    values: Some(HashMap::from([
      ("value".to_string(), num.to_string())
    ])),
    branches: None,
    args: None
  }
}