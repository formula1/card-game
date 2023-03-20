import { FunkRunner } from "../../../../types/Evaluator"
import { LangNode, ValueNode } from "../../../../types/ReusedStructs";
import { extractArray, packResult } from "../../utils";


export class RoundRunner implements FunkRunner {
  run(args: Array<LangNode> ): ValueNode {
    let nums = extractArray(args);
    return packResult(Math.round(nums[0]));
  }
}

export class CeilRunner implements FunkRunner {
  run(args: Array<LangNode> ): ValueNode {
    let nums = extractArray(args);
    return packResult(Math.ceil(nums[0]));
  }
}

export class FloorRunner implements FunkRunner {
  run(args: Array<LangNode> ): ValueNode {
    let nums = extractArray(args);
    return packResult(Math.floor(nums[0]));
  }
}

function random(): number{
  return Math.random();
}

export class FlipRunner implements FunkRunner {
  run(args: Array<LangNode> ): ValueNode {
    let nums = extractArray(args);
    if(nums[0] % 1 > 0 && random() > 0.5){
      nums[0] = nums[0] + 1;
    }
    return packResult(Math.floor(nums[0]));
  }
}

export class ProbRunner implements FunkRunner {
  run(args: Array<LangNode> ): ValueNode {
    let nums = extractArray(args);
    let remainder = nums[0] % 1;
    if(remainder > 0 && remainder > random()){
      nums[0] = nums[0] + 1;
    }
    return packResult(Math.floor(nums[0]));
  }
}
