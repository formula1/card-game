use std::collections::HashMap;

pub struct Token {
  pub token_type: String,
  pub values: HashMap<String, String>,
}

pub struct Node {
  pub node_type: String,
  pub values: Option<HashMap<String, String>>,
  pub branches: Option<HashMap<String, Node>>
}

impl Node {
  fn ValueNode(values: HashMap<String, String>) -> Node {
    return Node {
      node_type: "value".to_string(),
      values: Some(values),
      branches: None
    }
  }
  fn OperatorNode(branches: HashMap<String, Node>) -> Node {
    return Node {
      node_type: "operator".to_string(),
      values: None,
      branches: Some(branches)
    }
  }
  fn IdentifierNode(values: HashMap<String, String>) -> Node {
    return Node {
      node_type: "identifier".to_string(),
      values: None
      branches:
    }
  }
  fn AssignNode(values: HashMap<String, String>) -> Node {
    return Node {
      node_type: "assign".to_string(),
      values: None
      branches:
    }
  }
  fn CallNode(values: HashMap<String, String>) -> Node {
    return Node {
      node_type: "call".to_string(),
      values: None
      branches:
    }
  }
  fn FunctionNode(values: HashMap<String, String>) -> Node {
    return Node {
      node_type: "operator".to_string(),
      values: None
      branches:
    }
  }
}
