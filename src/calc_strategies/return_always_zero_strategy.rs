use crate::calc_base::{MathEvaluateError, MathParseError};
use crate::calc_base::value::Value;
use crate::calc_strategies::ICalculatorStrategy;

#[derive(Default, Debug)]
pub struct RichardStrategy{

}

impl ICalculatorStrategy for RichardStrategy {
    fn parse(&mut self, math_expr: &str) -> Result<(), MathParseError> {
        Ok(())
    }

    fn evaluate(&mut self) -> Result<Value, MathEvaluateError> {
        Ok(Value::Integer(1))
    }
}