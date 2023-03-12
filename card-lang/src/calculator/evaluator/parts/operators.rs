use std::collections::HashMap;

use crate::types::Evaluator::Operator;
use crate::types::Evaluator::OperatorRunner;
use crate::types::ReusedStructs::Node;
use crate::types::ReusedStructs::NodeType;


pub const operators: Vec<Operator> = vec![
  Operator{ id: "+".to_string(), runner: Box::new(AddRunner{}) },
  Operator{ id: "-".to_string(), runner: Box::new(SubRunner{}) },
  Operator{ id: "*".to_string(), runner: Box::new(MulRunner{}) },
  Operator{ id: "/".to_string(), runner: Box::new(DivRunner{}) },
  Operator{ id: "%".to_string(), runner: Box::new(ModRunner{}) },
  Operator{ id: "^".to_string(), runner: Box::new(ExpRunner{}) },
];

struct AddRunner{}
impl OperatorRunner for AddRunner {
  fn run(self, a: Node, b: Node )-> Node {
    let (a_num, b_num) = extractNumbers(a, b);
    return packResult(a_num + b_num);
  }
}

struct SubRunner{}
impl OperatorRunner for SubRunner {
  fn run(self, a: Node, b: Node )-> Node {
    let (a_num, b_num) = extractNumbers(a, b);
    return packResult(a_num - b_num);
  }
}

struct MulRunner{}
impl OperatorRunner for MulRunner {
  fn run(self, a: Node, b: Node )-> Node {
    let (a_num, b_num) = extractNumbers(a, b);
    return packResult(a_num * b_num);
  }
}

struct DivRunner{}
impl OperatorRunner for DivRunner {
  fn run(self, a: Node, b: Node )-> Node {
    let (a_num, b_num) = extractNumbers(a, b);
    return packResult(a_num / b_num);
  }
}

struct ModRunner{}
impl OperatorRunner for ModRunner {
  fn run(self, a: Node, b: Node )-> Node {
    let (a_num, b_num) = extractNumbers(a, b);
    return packResult(a_num % b_num);
  }
}

struct ExpRunner{}
impl OperatorRunner for ExpRunner {
  fn run(self, a: Node, b: Node )-> Node {
    let (a_num, b_num) = extractNumbers(a, b);
    return packResult(a_num.powf(b_num));
  }
}


pub fn extractNumbers(a: Node, b: Node) -> (f64, f64){
  if !matches!(a.node_type, NodeType::ValueNode) {
    panic!("a node_type is not ValueNode");
  }
  if !matches!(b.node_type, NodeType::ValueNode) {
    panic!("b node_type is not ValueNode");
  }
  let a_num = a.values.unwrap()["value"].parse::<f64>().unwrap();
  let b_num = b.values.unwrap()["value"].parse::<f64>().unwrap();
  return (a_num, b_num)
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