import { Constant } from "../../../types/Evaluator";
import { packResult } from "../utils";

export function getConstants(): Array<Constant> {
  return [
    {
      id: "pi",
      value: packResult(Math.PI)
    },
    {
      id: "e",
      value: packResult(Math.E)
    }
  ]
}

