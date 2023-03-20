import { FunkRunner } from "../../../../types/Evaluator";
import { LangNode, ValueNode } from "../../../../types/ReusedStructs";
import { extractArray, packResult } from "../../utils";

export class SinRunner implements FunkRunner {
  run(args: Array<LangNode> ): ValueNode {
    let nums = extractArray(args);
    return packResult(Math.sin(nums[0]));
  }
}

export class CosRunner implements FunkRunner {
  run(args: Array<LangNode> ): ValueNode {
    let nums = extractArray(args);
    return packResult(Math.cos(nums[0]));
  }
}

export class TanRunner implements FunkRunner {
  run(args: Array<LangNode> ): ValueNode {
    let nums = extractArray(args);
    return packResult(Math.tan(nums[0]));
  }
}

export class ASinRunner implements FunkRunner {
  run(args: Array<LangNode> ): ValueNode {
    let nums = extractArray(args);
    return packResult(Math.asin(nums[0]));
  }
}

export class ACosRunner implements FunkRunner {
  run(args: Array<LangNode> ): ValueNode {
    let nums = extractArray(args);
    return packResult(Math.acos(nums[0]));
  }
}

export class ATanRunner implements FunkRunner {
  run(args: Array<LangNode> ): ValueNode {
    let nums = extractArray(args);
    return packResult(Math.atan(nums[0]));
  }
}
