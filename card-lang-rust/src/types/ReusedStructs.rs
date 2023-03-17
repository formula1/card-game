use std::collections::HashMap;

#[derive(Clone)]
pub struct Token {
  pub token_type: String,
  pub values: HashMap<String, String>,
}

#[derive(Clone)]
pub struct Node {
  pub node_type: NodeType,
  pub values: Option<HashMap<String, String>>,
  pub branches: Option<HashMap<String, Node>>,
  pub args: Option<Vec<Node>>,
}

#[derive(Clone)]
pub enum NodeType {
  ValueNode,
  OperatorNode,
  IdentifierNode,
  AssignNode,
  CallNode,
  FunctionNode
}

impl Node {
  fn ValueNode(values: HashMap<String, String>) -> Node {
    return Node {
      node_type: NodeType::ValueNode,
      values: Some(values),
      branches: None,
      args: None,
    }
  }
  fn OperatorNode(branches: HashMap<String, Node>) -> Node {
    return Node {
      node_type: NodeType::OperatorNode,
      values: None,
      branches: Some(branches),
      args: None,
    }
  }
  fn IdentifierNode(values: HashMap<String, String>) -> Node {
    return Node {
      node_type: NodeType::IdentifierNode,
      values: None,
      branches: None,
      args: None,

    }
  }
  fn AssignNode(values: HashMap<String, String>) -> Node {
    return Node {
      node_type: NodeType::AssignNode,
      values: None,
      branches: None,
      args: None,
    }
  }
  fn CallNode(values: HashMap<String, String>) -> Node {
    return Node {
      node_type: NodeType::CallNode,
      values: None,
      branches:None,
      args: None,
    }
  }
  fn FunctionNode(values: HashMap<String, String>) -> Node {
    return Node {
      node_type: NodeType::FunctionNode,
      values: None,
      branches: None,
      args: None,
    }
  }
}
