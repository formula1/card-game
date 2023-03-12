
use std::collections::HashMap;
use std::f64::consts;

use crate::types::Evaluator::Constant;
use crate::types::ReusedStructs::Node;
use crate::types::ReusedStructs::NodeType;

pub const constants: Vec<Constant> = vec![
  Constant{
    id: "pi".to_string(),
    value: packResult(consts::PI)
  },
  Constant{
    id: "e".to_string(),
    value: packResult(consts::E)
  }
];


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

