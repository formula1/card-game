
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
      evaluator.constants.insert(value.id.clone(), value);
    }
    for value in variables {
      evaluator.variables.insert(value.id.clone(), value);
    }
    for value in prefixes {
      evaluator.prefixes.insert(value.id.clone(), value);
    }
    for value in operators {
      evaluator.operators.insert(value.id.clone(), value);
    }
    for value in functions {
      evaluator.functions.insert(value.id.clone(), value);
    }
    return evaluator;
  }

  fn addConst(&mut self, constant: Constant){
    self.constants.insert(constant.id.clone(), constant);
  }
  fn addVar(&mut self, variable: Variable){
    self.variables.insert(variable.id.clone(), variable);
  }
  fn addPre(&mut self, prefix: PrefixOperator){
    self.prefixes.insert(prefix.id.clone(), prefix);
  }
  fn addOp(&mut self, operator: Operator){
    self.operators.insert(operator.id.clone(), operator);
  }
  fn addFunk(&mut self, funk: Funk){
    self.functions.insert(funk.id.clone(), funk);
  }

  pub fn evaluate(self, parseTree: Vec<Node>) -> String{
    let mut output = "".to_string();
    let mut e: Self = self;
    let mut value: Node;
    for node in parseTree {
      (e, value) = e.parseNode(node);
      output.push_str(Evaluator::nodeToString(value).as_str());
      output.push_str("\n");
    }
    return output;
  }

  fn parseNode(self, node: Node) -> (Self, Node) {
    let mut e: Evaluator = self;
    let n = match node.node_type {
      NodeType::ValueNode=>{
        node
      }
      NodeType::OperatorNode=>{
        let values = node.values.unwrap();
        let op_name = values.get(&"type".to_string()).unwrap();

        let branches = node.branches.unwrap();
        let left = branches.get(&"left".to_string());
        let right: Node;
        (e, right) = self.parseNode(branches.get(&"right".to_string()).unwrap().clone());
        if left.is_none() {
          let prefixRunner = e.prefixes.get(op_name).unwrap().runner;
          prefixRunner.run(right)
        } else {
          let op = e.operators.get(&op_name.to_string()).unwrap().runner;
          let node: Node;
          (e, node) = e.parseNode(left.unwrap().clone());

          return (e, op.run(node, right));
        }
      }
      NodeType::IdentifierNode=>{
        let name = node.values.unwrap().get("value").unwrap().clone();
        let arguments = e.arguments;
        let constants = e.constants;
        let variables = e.variables;
        if arguments.contains_key(&name) {
          return (e, arguments.get(&name).unwrap().value.clone());
        } else if constants.contains_key(&name) {
          return (e, constants.get(&name).unwrap().value.clone());
        } else if variables.contains_key(&name) {
          return (e, variables.get(&name).unwrap().value.clone());
        } else {
          panic!("identifier {} doesn't exist", name);
        }
      }
      NodeType::AssignNode=>{
        let name = node.values.unwrap().get(&"name".to_string()).unwrap().clone();
        let value: Node;
        (e, value) = e.parseNode(
          node.branches.unwrap().get(&"value".to_string()).unwrap().clone()
        );
        e.variables.insert(name.clone(), Variable{ id: name.clone(), value: value.clone() });
        return (e, value)
      }
      NodeType::CallNode=>{
        let mut vals: Vec<Node> = vec![];
        for arg in node.args.unwrap().iter() {
          let result: Node;
          (e, result) = e.parseNode(arg.clone());
          vals.push(result);
        }
        let funk = e.functions.get(
          &node.values.unwrap().get("name").unwrap().clone()
        ).unwrap();
        return (e, funk.runner.run(vals));
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
    return (e, n);
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
        return values.get(&"value".to_string()).unwrap().clone();
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
  fn run(&self, a: Node)->Node;
}

pub struct Operator {
  id: String,
  runner: Box<dyn OperatorRunner>
}

pub trait OperatorRunner {
  fn run(&self, a: Node, b: Node)->Node;
}

pub struct Funk {
  id: String,
  numArgs: u128,
  runner: Box<dyn FunkRunner>
}

pub trait FunkRunner {
  fn run(&self, args: Vec<Node> )->Node;
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