import { LexerController, Tokenizer } from "../../../types/Lexer";

export class OperatorTokenizer implements Tokenizer {
  token_type = "operator";

  matchesChar(input: string){
    return matchesChar(input)
  }
  handleChar(c: string, lexer: LexerController){
    return handleChar(c, lexer);
  }
}

export function matchesChar(input: string){
  return /[+\-*/^%=(),]/.test(input);
}

function handleChar(c: string, lexer: LexerController){
  let l = lexer.addToken({ value: c });
  lexer.advance();
}

