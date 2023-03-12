use crate::types::Parser::Parser;
use crate::types::Parser::Prefix;
use crate::types::Parser::Infix;
use crate::types::Parser::Symbol;

#[path = "./symbols/open-parenthesis.rs"] mod open_paren_nud;
#[path = "./symbols/number.rs"] mod number;
#[path = "./symbols/identifier.rs"] mod identifier;
#[path = "./symbols/equal.rs"] mod equal;


pub fn createCalculatorParser() -> Parser {
  return Parser::new(
    vec![
      Prefix{ id: "-".to_string(), rbp: 7 }
    ],
    vec![
      Infix{ id: "^".to_string(), lbp: 6, rbp: Some(5), led: None },
      Infix{ id: "*".to_string(), lbp: 4, rbp: None, led: None },
      Infix{ id: "/".to_string(), lbp: 4, rbp: None, led: None },
      Infix{ id: "%".to_string(), lbp: 4, rbp: None, led: None },
      Infix{ id: "+".to_string(), lbp: 3, rbp: None, led: None },
      Infix{ id: "-".to_string(), lbp: 3, rbp: None, led: None },
      Infix{ id: "=".to_string(), lbp: 1, rbp: Some(2), led: Some(Box::new(equal::EqualLed{})) },
    ],
    vec![
      Symbol{ id: ",".to_string(), nud: None, lbp: None, led: None },
      Symbol{ id: ")".to_string(), nud: None, lbp: None, led: None },
      Symbol{ id: "(end)".to_string(), nud: None, lbp: None, led: None },
      Symbol{ id: "(".to_string(), nud: Some(Box::new(open_paren_nud::OpenParenthesisNud{})), lbp: None, led: None },
      Symbol{ id: "number".to_string(), nud: Some(Box::new(number::NumberNud{})), lbp: None, led: None },
      Symbol{ id: "identifier".to_string(), nud: Some(Box::new(identifier::IdentifierNud{})), lbp: None, led: None },
    ]
  );
}

