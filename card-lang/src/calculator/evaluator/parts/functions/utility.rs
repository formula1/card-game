use std::collections::HashMap;

use crate::types::Evaluator::Funk;
use crate::types::Evaluator::FunkRunner;

use crate::types::ReusedStructs::Node;
use crate::types::ReusedStructs::NodeType;

pub struct AbsRunner{}
impl FunkRunner for AbsRunner {
  fn run(&self, args: Vec<Node> )-> Node {
    let nums = extractNumbers(args);
    return packResult(nums[0].abs());
  }
}

pub struct MaxRunner{}
impl FunkRunner for MaxRunner {
  fn run(&self, args: Vec<Node> )-> Node {
    let nums = extractNumbers(args);
    return packResult(nums[0].max(nums[1]));
  }
}

pub struct MinRunner{}
impl FunkRunner for MinRunner {
  fn run(&self, args: Vec<Node> )-> Node {
    let nums = extractNumbers(args);
    return packResult(nums[0].min(nums[1]));
  }
}

pub fn extractNumbers(nums: Vec<Node>) -> Vec<f64> {
  return nums.iter().map(|node|-> f64 {
    if !matches!(node.node_type, NodeType::ValueNode) {
      panic!("an argument node_type is not ValueNode");
    } 
    node.values.clone().unwrap()["value"].parse::<f64>().unwrap()
  }).collect();
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