
use crate::types::Parser::Parser;
use crate::types::Parser::NudListener;
use crate::types::Parser::SymbolAndToken;
use crate::types::ReusedStructs::Node;

pub struct OpenParenthesisNud {}
impl NudListener for OpenParenthesisNud {
  fn run(self, symtok: SymbolAndToken, parser: Parser)->Node{
    let value = parser.expression(2);
    if parser.token().token.token_type != ")" {
      panic!("Expected closing parenthesis ')'");
    }
    parser.advance();
    return value;
  }
}
