import { FunkRunner } from "../../../../types/Evaluator";
import { LangNode, ValueNode } from "../../../../types/ReusedStructs";
import { extractArray, packResult } from "../../utils";

export class LogRunner implements FunkRunner{
  run(args: Array<LangNode> ): ValueNode {
    let nums = extractArray(args);
    return packResult(
      Math.log(nums[0])/Math.log(nums[1])
    );
  }
}
