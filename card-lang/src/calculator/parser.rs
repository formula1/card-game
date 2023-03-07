use crate::types::Parser;
use crate::types::Lexer;
use crate::types::ReusedStructs;


pub struct CalculatorParser {
  parser: Parser::Parser
}

pub const CalculatorParser = Parser::Parser::new();

impl CalculatorParser {
  fn new(tokens: Vec<ReusedStructs::Token>){
    let parser = CalculatorParser {
      parser: Parser::Parser {
        symbols: Parser::SymbolCollection::new(),
        tokens,
        i: 0,
      }
    };
    parser.prefix("-", 7);
    parser.infix("^", 6, 5);
    parser.infix("*", 4);
    parser.infix("/", 4);
    parser.infix("%", 4);
    parser.infix("+", 3);
    parser.infix("-", 3);
    parser.symbol(",");
    parser.symbol(")");
    parser.symbol("(end)");
    parser.symbol("(", || {
      let value = parser.expression(2);
      if parser.token().token_type != ")" {
        panic!("Expected closing parenthesis ')'");
      }
      parser.advance();
      return value;
    });
    parser.symbol("number", |number: u128| {
        return number;
    });
    parser.symbol("identifier", |name: String| {
      if (parser.token().type === "(") {
        var args = [];
        if (tokens[i + 1].type === ")") advance();
        else {
          do {
            advance();
            args.push(expression(2));
          } while (token().type === ",");
          if (token().type !== ")") throw "Expected closing parenthesis ')'";
        }
        advance();
        return {
          type: "call",
          args: args,
          name: name.value
        };
      }
      return name;
    });
    parser.infix("=", 1, 2, function (left) {
        if (left.type === "call") {
          for (var i = 0; i < left.args.length; i++) {
            if (left.args[i].type !== "identifier") throw "Invalid argument name";
          }
          return {
            type: "function",
            name: left.name,
            args: left.args,
            value: expression(2)
          };
        } else if (left.type === "identifier") {
          return {
            type: "assign",
            name: left.value,
            value: expression(2)
          };
        }
        else throw "Invalid lvalue";
    });

    return parser;
  }
}