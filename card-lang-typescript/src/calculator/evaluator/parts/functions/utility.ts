import { FunkRunner } from "../../../../types/Evaluator";
import { LangNode, ValueNode } from "../../../../types/ReusedStructs";
import { extractArray, packResult } from "../../utils";

export class AbsRunner implements FunkRunner {
  run(args: Array<LangNode> ): ValueNode {
    let nums = extractArray(args);
    return packResult(Math.abs(nums[0]));
  }
}

export class MaxRunner implements FunkRunner {
  run(args: Array<LangNode> ): ValueNode {
    let nums = extractArray(args);
    return packResult(Math.max(nums[0], nums[1]));
  }
}

export class MinRunner implements FunkRunner {
  run(args: Array<LangNode> ): ValueNode {
    let nums = extractArray(args);
    return packResult(Math.min(nums[0], nums[1]));
  }
}
