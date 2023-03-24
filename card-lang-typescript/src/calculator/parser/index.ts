
import { Parser, Prefix, Infix, ParserSymbol } from "../../types/Parser";

import { OpenParenthesisNud } from "./symbols/open-parenthesis";
import { NumberNud } from "./symbols/number";
import { IdentifierNud } from "./symbols/identifier";

import { EqualLed } from "./symbols/equal";


export class CalculatorParser extends Parser{
  constructor(){
    super(
      [
        { id: "-", rbp: 7 }
      ],
      [
        { id: "^", lbp: 6, rbp: 5, led: void 0 },
        { id: "*", lbp: 4, rbp: void 0, led: void 0 },
        { id: "/", lbp: 4, rbp: void 0, led: void 0 },
        { id: "%", lbp: 4, rbp: void 0, led: void 0 },
        { id: "+", lbp: 3, rbp: void 0, led: void 0 },
        { id: "-", lbp: 3, rbp: void 0, led: void 0 },
        { id: "=", lbp: 1, rbp: 2, led: new EqualLed() },
      ],
      [
        { id: ",", nud: void 0, lbp: void 0, led: void 0 },
        { id: ")", nud: void 0, lbp: void 0, led: void 0 },
        { id: "(end)", nud: void 0, lbp: void 0, led: void 0 },
        { id: "(", nud: new OpenParenthesisNud(), lbp: void 0, led: void 0 },
        { id: "number", nud: new NumberNud(), lbp: void 0, led: void 0 },
        { id: "identifier", nud: new IdentifierNud(), lbp: void 0, led: void 0 },
      ]
    )
  }
}

export const CALCULATOR_PARSER = new CalculatorParser()

