import { NudListener, ParserController, SymbolAndToken } from "../../../types/Parser";
import { LangNode, LangNodeType, Token } from "../../../types/ReusedStructs";

export class IdentifierNud implements NudListener {
  run(symtok: SymbolAndToken, parser: ParserController): LangNode{
    let token = symtok.token;
    let name = token.values?.name;
    if(name == void 0){
      throw new Error("Needed a name for identifier");
    }
    if(!isOpener(symtok.token)){
      return {
        node_type: LangNodeType.IdentifierNode,
        values: { name },
      }
    }

    var args: Array<LangNode> = [];
    let t = parser.nextToken();
    if(isCloser(t)){
      parser.advance();
    } else {
      while(true){
        parser.advance();
        args.push(parser.expression(2));
        let t: SymbolAndToken;
        if(!isComma(parser.token().token)){ break }
      }
      if(!isCloser(parser.token().token)){
        var value = parser.token().token.values.value;
        throw new Error(`Expected closing parenthesis ')' got ${value}`);
      };
    }
    parser.advance();
    return {
      node_type: LangNodeType.CallNode,
      values: { name },
      args: args,
    }
  }
}

function isOpener(token: Token): boolean {
  if(token.token_type != "operator"){
    return false
  }
  let str: String = token.values.value;
  if(str != "("){
    return false;
  }
  return true;
}

function isCloser(token: Token): boolean {
  if(token.token_type != "operator"){
    return false
  }
  let str = token.values.value;
  if(str != ")"){
    return false;
  }
  return true;
}

function isComma(token: Token): boolean {
  if(token.token_type != "operator"){
    return false
  }
  let str = token.values.value;
  if(str != ","){
    return false;
  }
  return true;

}
