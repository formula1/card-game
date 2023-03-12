
use crate::types::Evaluator;

#[path = "./parts/constants.rs"] mod calc_const;
#[path = "./parts/variables.rs"] mod calc_var;
#[path = "./parts/functions/mod.rs"] mod calc_funk;
#[path = "./parts/operators.rs"] mod calc_op;
#[path = "./parts/prefixes.rs"] mod calc_pre;

pub const CalculatorLexer: Evaluator::Evaluator = Evaluator::Evaluator::new(
  calc_const::constants,
  calc_var::variables,
  calc_pre::prefixes,
  calc_op::operators,
  calc_funk::functions,
);
