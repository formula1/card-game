import { IdentifierNode, LangNode, LangNodeType } from "../../../types/ReusedStructs";
import { ParserController, LedListener } from "../../../types/Parser";

export class EqualLed implements LedListener {

  run(left: LangNode, parser: ParserController): LangNode {
    switch(left.node_type){
      case LangNodeType.CallNode: {
        let args: Array<IdentifierNode> = left.args.map((arg)=>{
          if (arg.node_type !== LangNodeType.IdentifierNode){
            throw new Error("Invalid argument name");
          }
          return arg
        });
        for(var i = 0; i < left.args.length; i++){
        }
        return {
          node_type: LangNodeType.FunctionNode,
          values: { name: left.values.name },
          args: args,
          branches: { right: parser.expression(2) }
        }
      }
      case LangNodeType.IdentifierNode: {
        return {
          node_type: LangNodeType.AssignNode,
          values: { name: left.values.name },
          branches: { right: parser.expression(2) }
        }
      }
      default: {
        throw new Error("Invalid Node Type")
      }
    }
  }
}

