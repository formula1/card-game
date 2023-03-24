import { Evaluator } from "../../types/Evaluator";

import { getConstants } from "./parts/constants";
import { getFunctions } from "./parts/functions";
import { getOperators } from "./parts/operators";
import { getPrefixes } from "./parts/prefixes";

export class CalculatorEvaluator extends Evaluator {
  constructor(){
    super(
      getConstants(),
      getPrefixes(),
      getOperators(),
      getFunctions(),
    )
  }
}

export const CALCULATOR_EVALUATOR = new CalculatorEvaluator();
