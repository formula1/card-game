

import { Token, LangNode, LangNodeType } from "./ReusedStructs";

export class Parser {
  symbols: SymbolCollection = new SymbolCollection();
  tokens: Array<Token> = []


  constructor(prefixes: Array<Prefix>, infixes: Array<Infix>, symbols: Array<ParserSymbol>){
    for(var prefix of prefixes){
      this.prefix(prefix.id, prefix.rbp);
    }
    for(var infix of infixes){
      this.infix(infix.id, infix.lbp, infix.rbp, infix.led);
    }
    for(var symbol of symbols){
      this.symbol(symbol.id, symbol.nud, symbol.lbp, symbol.led);
    }
  }
  symbol(id: string, nud: void | NudListener, lbp: void | number, led: void | LedListener){
    this.symbols.setSymbol(id, nud, lbp, led);
  }

  infix(id: string, lbp: number, rbp_op: void | number, led_maybe: void | LedListener) {
    let rbp = typeof rbp_op != "undefined" ? rbp_op : lbp;
    let led = led_maybe != void 0 ? led_maybe : (
      new DefaultInfix( id, rbp )
    );
    this.symbols.setSymbol(id, void 0, lbp, led);
  }
  
  prefix(id: string, rbp: number) {
    let pre = new DefaultPrefix(
      id, rbp
    );
    this.symbols.setSymbol(id, pre, void 0, void 0);
  }


  parse(tokens: Array<Token>): Array<LangNode>{
    this.tokens = tokens;
    var parseTree: Array<LangNode> = [];
    let controller = new ParserController(this);

    while(controller.i < tokens.length) {
      var t = controller.token();
      if(typeof t == "undefined") { break; }
      if(t.token.token_type == "(end)") { break; }
      let n = controller.expression(0);
      parseTree.push(n);
    }

    return parseTree;
  }


}

export class ParserController {
  i: number = 0;
  constructor(private parser: Parser){}

  token(): void | SymbolAndToken {
    var token = this.parser.tokens[this.i];
    if(typeof token == "undefined"){
      return void 0;
    }
    return (
      this.parser.symbols.interpretToken(token)
    );
  }
  nextToken(): void | Token {
    return (
      this.parser.tokens[this.i + 1]
    );
  }
  advance() {
    this.i += 1;
    if(this.i == this.parser.tokens.length){
      // console.warn("no tokens left")
    }
  }

  expression(rbp: number): LangNode {
    var t1 = this.token();
    if(typeof t1 == "undefined"){
      throw new Error("expected token, got undefined");
    }
    this.advance();
    let nud = t1.symbol.nud;
    if(typeof nud == "undefined"){
      throw new Error(`Unexpected token: ${t1.token.token_type}`);
    }
    var left = nud.run(t1, this);
    while(true) {
      let t = this.token();
      if(typeof t == "undefined"){ break; }
      if(rbp >= (t.symbol.lbp || 0)) { break; }
      this.advance();

      let led = t.symbol.led;
      if(typeof led == "undefined"){
        throw new Error(`Unexpected token: ${t.token.token_type}`)
      }
      left = led.run(left, this);
    };
    return left;
  }
}

type Optional<Real> = {
  [K in keyof Real]: void | Real[K]
}

export type ParserSymbol = {
  id: string,
} & Partial<Optional<{
  nud: NudListener,
  lbp: number,
  led: LedListener,
}>>

export interface NudListener {
  run(symtok: SymbolAndToken, parser: ParserController): LangNode;
}

export interface LedListener {
  run(node: LangNode, parser: ParserController): LangNode;
}

export type Prefix = {
  id: string,
  rbp: number
}

export type Infix = {
  id: string,
  lbp: number,
} & Partial<Optional<{
  rbp: number,
  led: LedListener
}>>

class SymbolCollection {
  symbols: { [key: string]: ParserSymbol } = {};

  setSymbol(id: string, nud: void | NudListener, lbp: void | number, led: void | LedListener){
    let maybe_sym = this.symbols[id];
    if(maybe_sym == void 0){
      this.symbols[id] = {
        id, nud, lbp, led
      };
    } else {
      let sym = maybe_sym;
      if(sym.nud == void 0){ sym.nud = nud; }
      if(sym.lbp == void 0){ sym.lbp = lbp; }
      if(sym.led == void 0){ sym.led = led; }
    }
  }
  interpretToken(token: Token): SymbolAndToken {
    let token_type: string = token.token_type == "operator" ?(
      token.values["value"]
    ) : (
      token.token_type
    )
    let sym = this.symbols[token_type];
    return {
      token: token,
      symbol: sym
    }
  }
}


export type SymbolAndToken = {
  symbol: ParserSymbol,
  token: Token
}

class DefaultPrefix implements NudListener {
  constructor(
    public symbol_id: string,
    public rbp: number
  ){}

  run(symtok: SymbolAndToken, parser: ParserController): LangNode {

    let values = { type: this.symbol_id };
    let node = parser.expression(this.rbp);
    let branches = { right: node }

    return {
      node_type: LangNodeType.OperatorNode,
      values: values,
      branches: branches,
    }
  }
}

class DefaultInfix implements LedListener {
  constructor(
    public symbol_id: string,
    public rbp: number
  ){}
  run(left: LangNode, parser: ParserController): LangNode {
  
    let values = { type: this.symbol_id };
    let node = parser.expression(this.rbp);
    let branches = {
      left: left,
      right: node
    }
  
    return {
      node_type: LangNodeType.OperatorNode,
      values: values,
      branches: branches,
    }
  }
}

