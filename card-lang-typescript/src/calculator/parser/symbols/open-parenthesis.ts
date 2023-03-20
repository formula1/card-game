import { NudListener, ParserController, SymbolAndToken } from "../../../types/Parser";
import { LangNode } from "../../../types/ReusedStructs";

export class OpenParenthesisNud implements NudListener {
  run(symtok: SymbolAndToken, parser: ParserController): LangNode{
    let n = parser.expression(2);
    let t = parser.token();
    if(t.token.token_type != ")"){
      throw new Error(`Expected closing parenthesis ')', got ${t.token.token_type}`);
    }
    parser.advance();
    return n;
  }
}
