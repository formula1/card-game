import { LexerController, Tokenizer } from "../../../types/Lexer";

export class WhiteSpaceTokenizer implements Tokenizer{
  token_type = "whitespace"
  matchesChar(input: string){
    return matchesChar(input)
  }
  handleChar(c: string, lexer: LexerController){
    return handleChar(c, lexer);
  }

}

export function matchesChar(input: string){
  return /[\s]/.test(input);
}

function handleChar(_: string, lexer: LexerController){
  lexer.advance();
}


// impl lex::Tokenizer for WhiteSpaceTokenizer {
//   fn token_typeStatic()->String {
//       return "whitespace".to_string()
//   }
//   fn matchesTypeStatic(self, input: char){
//     let re = Regex::new(r"\s]");
//     return re.is_match(input);
//   }
//   fn handleChar(
//     self,
//     initial_char: char,
//     advance: fn()->char,
//     addToken: fn(value: HashMap<String, String>)->(),
//   ){
//     advance();
//   }

// }