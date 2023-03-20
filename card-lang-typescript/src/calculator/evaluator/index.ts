import { Evaluator } from "../../types/Evaluator";

import { getConstants } from "./parts/constants";
import { getFunctions } from "./parts/functions";
import { getOperators } from "./parts/operators";
import { getPrefixes } from "./parts/prefixes";

export const CALCULATOR_EVALUATOR = new Evaluator(
  getConstants(),
  getPrefixes(),
  getOperators(),
  getFunctions(),
)
