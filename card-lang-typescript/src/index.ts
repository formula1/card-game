

import { CalculatorLexer, CALCULATOR_LEXER } from "./calculator/lexer";
import { CalculatorParser, CALCULATOR_PARSER } from "./calculator/parser";
import { CalculatorEvaluator, CALCULATOR_EVALUATOR } from "./calculator/evaluator";

export {
  CalculatorLexer,
  CalculatorParser,
  CalculatorEvaluator,
  CALCULATOR_LEXER,
  CALCULATOR_PARSER,
  CALCULATOR_EVALUATOR,
}

export function run(text: string){
  var lexer = new CalculatorLexer()
  var tokens = lexer.tokenizeString(text);
  
  var parser = new CalculatorParser();
  var nodes = parser.parse(tokens);

  var evaluator = new CalculatorEvaluator()
  var result = evaluator.evaluate(nodes);
  return result;
}