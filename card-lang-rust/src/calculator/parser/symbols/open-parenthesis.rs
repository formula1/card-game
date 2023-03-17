
use crate::types::Parser::Parser;
use crate::types::Parser::NudListener;
use crate::types::Parser::SymbolAndToken;
use crate::types::ReusedStructs::Node;

pub struct OpenParenthesisNud {}
impl NudListener for OpenParenthesisNud {
  fn run(&self, symtok: SymbolAndToken, parser: Parser)->Node{
    let (p, n) = parser.expression(2);
    let (p, t) = p.token();
    if t.token.token_type != ")" {
      panic!("Expected closing parenthesis ')'");
    }
    p.advance();
    return n;
  }
}
