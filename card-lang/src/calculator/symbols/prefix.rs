
use crate::types::Lexer::Lexer;
use crate::types::Parser::Parser;
use crate::types::Parser::Symbol;
use crate::types::ReusedStructs::Node;

struct Negative {
  parser: Parser
}
impl Symbol for Negative {
  fn own_parser(self)->Parser {
    return self.parser()
  }
  fn get_id(self)->String {
    return "-"
  }
  fn nud(self)-> Node {
    return Node {
      node_type: "prefix",
      values: None,
      branches: HashMap::from([
        ("left", self.own_parser().expression())
      ]),
    }
  }
  fn lbp(self)->u128;
  fn led(self, node: Option<Node>)->Node;
}
