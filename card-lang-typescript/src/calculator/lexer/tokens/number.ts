import { LexerController, Tokenizer } from "../../../types/Lexer";

export class DigitTokenizer implements Tokenizer {
  token_type = "number";
  matchesChar(input: string){
    return matchesChar(input)
  }
  handleChar(c: string, lexer: LexerController){
    return handleChar(c, lexer);
  }
}

export function matchesChar(input: string){
  return /[0-9]/.test(input);
}

function handleChar(initial_char: string, lexer: LexerController){
  var num = initial_char;
  var c: string | void = initial_char;
  let is_float = "0";

  collectNumber();

  if(c == '.'){
    is_float = "1";
    num += c;
    collectNumber()
  }
  lexer.addToken({
    is_float: is_float,
    value: num
  });

  function collectNumber(){
    while(true) {
      c = lexer.advance();
      if(typeof c == "undefined"){ break; }
      if(!matchesChar(c)){ break; }
      num += c;
    }
  }
}


