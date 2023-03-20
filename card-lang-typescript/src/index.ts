

import { CALCULATOR_PARSER } from "./calculator/parser";
import { CALCULATOR_LEXER } from "./calculator/lexer";
import { CALCULATOR_EVALUATOR } from "./calculator/evaluator";


export function run(text: string){
  var tokens = CALCULATOR_LEXER.tokenizeString(text);
  var nodes = CALCULATOR_PARSER.parse(tokens);
  var result = CALCULATOR_EVALUATOR.evaluate(nodes);
  return result;
}