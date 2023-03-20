import { Operator, OperatorRunner } from "../../../types/Evaluator";
import { LangNode, ValueNode } from "../../../types/ReusedStructs";
import { packResult, extractPair } from "../utils";


export function getOperators(): Array<Operator> {
  return [
    { id: "+", runner: new AddRunner() },
    { id: "-", runner: new SubRunner() },
    { id: "*", runner: new MulRunner() },
    { id: "/", runner: new DivRunner() },
    { id: "%", runner: new ModRunner() },
    { id: "^", runner: new ExpRunner() },
  ]
}

class AddRunner implements OperatorRunner{
  run(a: LangNode, b: LangNode ): ValueNode {
    let [a_num, b_num] = extractPair(a, b);
    return packResult(a_num + b_num);
  }
}

class SubRunner implements OperatorRunner{
  run(a: LangNode, b: LangNode ): ValueNode {
    let [a_num, b_num] = extractPair(a, b);
    return packResult(a_num - b_num);
  }
}

class MulRunner implements OperatorRunner {
  run(a: ValueNode, b: ValueNode ): ValueNode {
    let [a_num, b_num] = extractPair(a, b);
    return packResult(a_num * b_num);
  }
}

class DivRunner implements OperatorRunner {
  run(a: ValueNode, b: ValueNode ): ValueNode {
    let [a_num, b_num] = extractPair(a, b);
    return packResult(a_num / b_num);
  }
}

class ModRunner implements OperatorRunner {
  run(a: ValueNode, b: ValueNode ): ValueNode {
    let [a_num, b_num] = extractPair(a, b);
    return packResult(a_num % b_num);
  }
}

class ExpRunner implements OperatorRunner {
  run(a: ValueNode, b: ValueNode ): ValueNode {
    let [a_num, b_num] = extractPair(a, b);
    return packResult(Math.pow(a_num, b_num));
  }
}


