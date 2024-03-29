import { Lexer } from "../../types/Lexer";

import { DigitTokenizer } from "./tokens/number";
import { IdentifierTokenizer } from "./tokens/identifier";
import { OperatorTokenizer } from "./tokens/operator";
import { WhiteSpaceTokenizer } from "./tokens/whitespace";

// https://www.codeproject.com/Articles/345888/How-to-Write-a-Simple-Interpreter-in-JavaScript


export const CALCULATOR_LEXER = new Lexer(
  [
    new WhiteSpaceTokenizer(),
    new OperatorTokenizer(),
    new DigitTokenizer(),
    new IdentifierTokenizer(),
  ]
);

