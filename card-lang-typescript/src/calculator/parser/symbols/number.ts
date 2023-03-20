import { NudListener, ParserController, SymbolAndToken } from "../../../types/Parser";
import { LangNode, LangNodeType } from "../../../types/ReusedStructs";

export class NumberNud implements NudListener {
  run(symtok: SymbolAndToken, _: ParserController): LangNode {
    return {
      node_type: LangNodeType.ValueNode,
      values: {
        type: "number",
        is_float: symtok.token.values.is_float == "1",
        value: symtok.token.values.value
      }
    };
  }
}
