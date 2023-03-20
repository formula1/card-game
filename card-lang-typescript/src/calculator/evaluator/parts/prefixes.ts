
import { PrefixOperator, PrefixRunner } from "../../../types/Evaluator";
import { LangNode, LangNodeType, ValueNode } from "../../../types/ReusedStructs";
import { packResult, extractNumber } from "../utils";

export function getPrefixes(): Array<PrefixOperator>{
  return [
    { id: "-", runner: new SubRunner() },
  ];
}

class SubRunner implements PrefixRunner{
  run(a: LangNode): ValueNode {
    let a_num = extractNumber(a);
    return packResult(-1 * a_num);
  }
}

