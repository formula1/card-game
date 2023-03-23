import { LexerController, Tokenizer } from "../../../types/Lexer";

import { matchesChar as opMatchesChar } from "./operator";
import { matchesChar as digMatchesChar } from "./number";
import { matchesChar as wsMatchesChar } from "./whitespace";

export class IdentifierTokenizer implements Tokenizer {
  token_type = "identifier";

  matchesChar(input: string){
    return matchesChar(input)
  }
  handleChar(c: string, lexer: LexerController){
    return handleChar(c, lexer);
  }

}

function matchesChar(input: string){
  return !opMatchesChar(input) && !digMatchesChar(input) && !wsMatchesChar(input);
}

function handleChar(initial_char: string, lexer: LexerController) {
  var c: string | void;
  let identity = initial_char;
  while(true) {
    c = lexer.advance();
    if(typeof c == "undefined" || !matchesChar(c)){ break; }
    identity += c;
  }
  lexer.addToken({ name: identity });
  lexer.advance();
}