import { Funk } from "../../../../types/Evaluator";

import { LogRunner } from "./operations";
import { CeilRunner, FlipRunner, FloorRunner, ProbRunner, RoundRunner } from "./round";
import { ACosRunner, ASinRunner, ATanRunner, CosRunner, SinRunner, TanRunner } from "./trig";
import { AbsRunner, MaxRunner, MinRunner } from "./utility";

export function getFunctions(): Array<Funk> {
  return [
    // Operations without a sign
    { id: "log", numArgs: 2, runner: new LogRunner() },

    // Utility Functions
    { id: "abs", numArgs: 1, runner: new AbsRunner() },
    { id: "max", numArgs: 2, runner: new MaxRunner() },
    { id: "min", numArgs: 2, runner: new MinRunner() },

    // Trigonometry
    { id: "sin", numArgs: 1, runner: new SinRunner() },
    { id: "cos", numArgs: 1, runner: new CosRunner() },
    { id: "tan", numArgs: 1, runner: new TanRunner() },
    { id: "asin", numArgs: 1, runner: new ASinRunner() },
    { id: "acos", numArgs: 1, runner: new ACosRunner() },
    { id: "atan", numArgs: 1, runner: new ATanRunner() },

    // Rounding
    { id: "round", numArgs: 1, runner: new RoundRunner() },
    { id: "ceil", numArgs: 1, runner: new CeilRunner() },
    { id: "floor", numArgs: 1, runner: new FloorRunner() },
    { id: "flip_round", numArgs: 1, runner: new FlipRunner() },
    { id: "prob_round", numArgs: 1, runner: new ProbRunner() },

      // Not used
    /*
      exp: Math.exp, (use constant E and ^)
      sqrt: Math.sqrt, (use ^(1/2))
    */

  ];
}
