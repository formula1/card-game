
import { Token } from "./ReusedStructs";

export class Lexer {
  current_tokenizer: string = "";

  input_chars: Array<string> = [];
  char_index: number = 0;
  tokens: Array<Token> = [];
  controller = new LexerController(this);

  constructor(
    private tokenizers: Array<Tokenizer>
  ){}
  reset(){
    this.current_tokenizer = "";
    this.input_chars = [];
    this.char_index = 0;
    this.tokens = [];
    return this;
  }
  tokenizeString(input_str: string): Array<Token>{
    this.reset();
    this.input_chars = Array.from(input_str);

    let tokenizers = this.tokenizers;
    var controller = this.controller;

    while(this.char_index < this.input_chars.length){
      var usedTokenizer = false;
      for(let t of tokenizers){
        if(!t.matchesChar(controller.current_char)){
          continue;
        }
        usedTokenizer = true;
        this.current_tokenizer = t.token_type;
        t.handleChar(controller.current_char, controller);
      }
      if(usedTokenizer == false){
        throw new Error(`invalid token ${controller.current_char}`);
      }
    }
    return this.tokens;
  }
}

export class LexerController {
  constructor(private lexer: Lexer){}
  get current_char(){
    return this.lexer.input_chars[this.lexer.char_index];
  }
  advance(): string{
    this.lexer.char_index++
    return this.current_char;
  }
  addToken(values: {[key: string]: string}){
    this.lexer.tokens.push({
      token_type: this.lexer.current_tokenizer,
      values: values
    });
    return self;
  }
}

export interface Tokenizer {
  token_type: string;
  matchesChar(c: string): boolean;
  handleChar(c: string, lexer: LexerController): void;
}

// pub struct Tokenizer<Adv, Tok>{
//   token_type: String,
//   matchesChar: fn(c: char)->bool,
//   handleChar: fn(
//     initial_char: char,
//     advance: Adv,
//     addToken: Tok,
//   )->Result<(), String> where Adv: Fn()->char, Tok: Fn(HashMap<String, String>)->()
// }

// advance: impl Fn()->char,
// addToken: fn(values: HashMap<String, String>)->(),

