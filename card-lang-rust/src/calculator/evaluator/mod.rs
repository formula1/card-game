
use crate::types::Evaluator;

#[path = "./parts/constants.rs"] mod calc_const;
#[path = "./parts/variables.rs"] mod calc_var;
#[path = "./parts/functions/mod.rs"] mod calc_funk;
#[path = "./parts/operators.rs"] mod calc_op;
#[path = "./parts/prefixes.rs"] mod calc_pre;

pub fn createCalculatorEvaluator() -> Evaluator::Evaluator{
  return Evaluator::Evaluator::new(
    calc_const::getConstants(),
    calc_var::variables,
    calc_pre::getPrefixes(),
    calc_op::getOperators(),
    calc_funk::getFunctions(),
  )
}
