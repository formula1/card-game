use std::collections::HashMap;

use crate::types::Evaluator::Funk;
use crate::types::Evaluator::FunkRunner;
use crate::types::ReusedStructs::Node;
use crate::types::ReusedStructs::NodeType;

#[path = "./operations.rs"] mod ops;
#[path = "./trig.rs"] mod trig;
#[path = "./round.rs"] mod round;
#[path = "./utility.rs"] mod util;

pub fn getFunctions() -> Vec<Funk>{
  return vec![
    // Operations without a sign
    Funk { id: "log".to_string(), numArgs: 2, runner: Box::new(ops::LogRunner{}) },

    // Utility Functions
    Funk { id: "abs".to_string(), numArgs: 1, runner: Box::new(util::AbsRunner{}) },
    Funk { id: "max".to_string(), numArgs: 2, runner: Box::new(util::MaxRunner{}) },
    Funk { id: "min".to_string(), numArgs: 2, runner: Box::new(util::MinRunner{}) },

    // Trigonometry
    Funk { id: "sin".to_string(), numArgs: 1, runner: Box::new(trig::SinRunner{}) },
    Funk { id: "cos".to_string(), numArgs: 1, runner: Box::new(trig::CosRunner{}) },
    Funk { id: "tan".to_string(), numArgs: 1, runner: Box::new(trig::TanRunner{}) },
    Funk { id: "asin".to_string(), numArgs: 1, runner: Box::new(trig::ASinRunner{}) },
    Funk { id: "acos".to_string(), numArgs: 1, runner: Box::new(trig::ACosRunner{}) },
    Funk { id: "atan".to_string(), numArgs: 1, runner: Box::new(trig::ATanRunner{}) },

    // Rounding
    Funk { id: "round".to_string(), numArgs: 1, runner: Box::new(round::RoundRunner{}) },
    Funk { id: "ceil".to_string(), numArgs: 1, runner: Box::new(round::CeilRunner{}) },
    Funk { id: "floor".to_string(), numArgs: 1, runner: Box::new(round::FloorRunner{}) },
    Funk { id: "flip_round".to_string(), numArgs: 1, runner: Box::new(round::FlipRunner{}) },
    Funk { id: "prob_round".to_string(), numArgs: 1, runner: Box::new(round::ProbRunner{}) },

      // Not used
    /*
      exp: Math.exp, (use constant E and ^)
      sqrt: Math.sqrt, (use ^(1/2))
    */

  ];
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
