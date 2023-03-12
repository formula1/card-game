
use std::collections::HashMap;

use crate::types::ReusedStructs::Node;
use crate::types::ReusedStructs::NodeType;

pub struct Evaluator {
  constants: HashMap<String, Constant>,
  variables: HashMap<String, Variable>,
  arguments: HashMap<String, Variable>,
  prefixes: HashMap<String, PrefixOperator>,
  operators: HashMap<String, Operator>,
  functions: HashMap<String, Funk>,
}

impl Evaluator {
  pub fn new(
    constants: Vec<Constant>,
    variables: Vec<Variable>,
    prefixes: Vec<PrefixOperator>,
    operators: Vec<Operator>,
    functions: Vec<Funk>,
  ) -> Evaluator {
    let mut evaluator = Evaluator {
      constants: HashMap::new(),
      variables: HashMap::new(),
      arguments: HashMap::new(),
      prefixes: HashMap::new(),
      operators: HashMap::new(),
      functions: HashMap::new(),
    };
    for value in constants {
      evaluator.constants.insert(value.id, value);
    }
    for value in variables {
      evaluator.variables.insert(value.id, value);
    }
    for value in prefixes {
      evaluator.prefixes.insert(value.id, value);
    }
    for value in operators {
      evaluator.operators.insert(value.id, value);
    }
    for value in functions {
      evaluator.functions.insert(value.id, value);
    }
    return evaluator;
  }

  fn addConst(&mut self, id: String, value: Node){
    self.constants.insert(id, Constant{ id, value });
  }
  fn addVar(&mut self, id: String, value: Node){
    self.variables.insert(id, Variable{ id, value });
  }
  fn addPre(&mut self, id: String, runner: Box<dyn PrefixRunner>){
    self.prefixes.insert(id, PrefixOperator{id, runner});
  }
  fn addOp(&mut self, id: String, runner: Box<dyn OperatorRunner>){
    self.operators.insert(id, Operator{id, runner});
  }
  fn addFunk(&mut self, id: String, numArgs: u128, runner: Box<dyn FunkRunner>){
    self.functions.insert(id, Funk{ id, numArgs, runner });
  }

  pub fn evaluate(self, parseTree: Vec<Node>) -> String{
    let output = "".to_string();
    for node in parseTree {
      let value = self.parseNode(node);
      output.push_str(self.nodeToString(value).as_str());
      output.push_str("\n");
    }
    return output;
  }

  fn parseNode(self, node: Node) -> Node{
    return match node.node_type {
      NodeType::ValueNode=>{
        return node
      }
      NodeType::OperatorNode=>{
        let op_name = node.values.unwrap().get(&"type".to_string()).unwrap();
        let op = self.operators[&op_name.to_string()];
        let left = node.branches.unwrap().get(&"left".to_string());
        let right = node.branches.unwrap().get(&"right".to_string()).unwrap();
        if left.is_none() {
          self.prefixes[op_name].runner.run(self.parseNode(*right))
        } else {
          return self.operators[op_name].runner.run(
            self.parseNode(*left.unwrap()),
            self.parseNode(*right)
          );
        }
      }
      NodeType::IdentifierNode=>{
        let name = node.values.unwrap().get("value").unwrap();
        if self.arguments.contains_key(name) {
          return self.arguments.get(name).unwrap().value;
        } else if self.constants.contains_key(name) {
          return self.constants.get(name).unwrap().value;
        } else if self.variables.contains_key(name) {
          return self.variables.get(name).unwrap().value;
        } else {
          panic!("identifier {} doesn't exist", name);
        }
      }
      NodeType::AssignNode=>{
        let name = *node.values.unwrap().get(&"name".to_string()).unwrap();
        let value = self.parseNode(*node.branches.unwrap().get(&"value".to_string()).unwrap());
        self.variables.insert(name, Variable{ id: name, value });
        return value
      }
      NodeType::CallNode=>{
        let vals: Vec<Node> = node.args.unwrap().iter()
        .map(|arg: &Node|-> Node { return self.parseNode(*arg) })
        .collect();
        return self.functions.get(node.values.unwrap().get("name").unwrap()).unwrap().runner.run(vals);
      }
      NodeType::FunctionNode=>{
        panic!("Haven't enabled function nodes");
        /*
        let name = node.values.unwrap().get("name").unwrap();

        let names = node.args.unwrap().iter()
        .map(|node| -> String { return *node.values.unwrap().get("name").unwrap() })
        .collect();

        let node = Funk{
          id: *name,
          numArgs: node.args.unwrap().len() as u128,
          runner: Box::new(CreatedFunk {
            evaluator: self,
            arg_names: names,
            node: node
          })
        };

        self.functions.insert(*name, node);
        */
      }
      _=>{
        panic!("invalid node type")
      }
    };
  }

  fn nodeToString(self, node: Node) -> String {
    if !matches!(node.node_type, NodeType::ValueNode) {
      panic!("node_type is not ValueNode");
    }
    let values = node.values.unwrap();
    match values["type"].as_str() {
      "boolean" => {
        if values["is_float"].as_str() == "1" {
          return "True".to_string();
        } else {
          return "False".to_string();
        }
      }
      "number" => {
        if values["is_float"].as_str() == "1" {
          return values["value"].as_str().parse::<f64>().unwrap().to_string()
        } else {
          return values["value"].as_str().parse::<i64>().unwrap().to_string()
        }
      }
      "string" => {
        return values["value"];
      }
      _ => {
        panic!("value typ")
      }
    }
  }
  fn valueToString(self, val: Value) -> String{
    match val {
      Value::BooleanValue(v) => {
        return if v { "True".to_string() } else { "False".to_string() };
      }
      Value::NumberValue(v) => {
        return v.to_string();
      }
      Value::StringValue(v) => {
        return v;
      }
      _ => {
        panic!("invalid value typ")
      }
    }
  }
}

pub enum Value {
  BooleanValue(bool),
  NumberValue(f64),
  StringValue(String),
}

pub struct Constant {
  id: String,
  value: Node
}

pub struct Variable {
  id: String,
  value: Node
}

pub struct PrefixOperator {
  id: String,
  runner: Box<dyn PrefixRunner>
}

pub trait PrefixRunner {
  fn run(self, a: Node)->Node;
}

pub struct Operator {
  id: String,
  runner: Box<dyn OperatorRunner>
}

pub trait OperatorRunner {
  fn run(self, a: Node, b: Node)->Node;
}

pub struct Funk {
  id: String,
  numArgs: u128,
  runner: Box<dyn FunkRunner>
}

pub trait FunkRunner {
  fn run(self, args: Vec<Node> )->Node;
}

struct CreatedFunk {
  evaluator: Evaluator,
  arg_names: Vec<String>,
  node: Node
}

// impl FunkRunner for CreatedFunk {
//   fn run(self, args: Vec<Node>) -> Node {
//     for (i, arg_name) in self.arg_names.iter().enumerate() {
//       self.evaluator.arguments.insert(
//         *arg_name, Variable{ id: *arg_name, value: args[i] }
//       );
//     }
//     let ret = self.evaluator.parseNode(*self.node.branches.unwrap().get("block").unwrap());
//     self.evaluator.arguments.clear();
//     return ret;
//   }
// }