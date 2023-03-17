use std::collections::HashMap;

use crate::types::Evaluator::FunkRunner;

use crate::types::ReusedStructs::Node;
use crate::types::ReusedStructs::NodeType;

pub struct RoundRunner{}
impl FunkRunner for RoundRunner {
  fn run(&self, args: Vec<Node> )-> Node {
    let nums = extractNumbers(args);
    return packResult(nums[0].round());
  }
}

pub struct CeilRunner{}
impl FunkRunner for CeilRunner {
  fn run(&self, args: Vec<Node> )-> Node {
    let nums = extractNumbers(args);
    return packResult(nums[0].ceil());
  }
}

pub struct FloorRunner{}
impl FunkRunner for FloorRunner {
  fn run(&self, args: Vec<Node> )-> Node {
    let nums = extractNumbers(args);
    return packResult(nums[0].floor());
  }
}

fn random() -> f64 {
  return 0_f64;
}

pub struct FlipRunner{}
impl FunkRunner for FlipRunner {
  fn run(&self, args: Vec<Node> )-> Node {
    let mut nums = extractNumbers(args);
    if nums[0] % 1_f64 > 0_f64 && random() > 0.5 {
      nums[0] = nums[0] + 1_f64;
    }
    return packResult(nums[0]);
  }
}

pub struct ProbRunner{}
impl FunkRunner for ProbRunner {
  fn run(&self, args: Vec<Node> )-> Node {
    let mut nums = extractNumbers(args);
    let remainder = nums[0] % 1_f64;
    if remainder > 0_f64 && random() < remainder {
      nums[0] = nums[0] + 1_f64;
    }
    return packResult(nums[0]);
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