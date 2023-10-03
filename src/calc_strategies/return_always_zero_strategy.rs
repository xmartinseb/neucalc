use std::marker::PhantomData;
use crate::calc_base::{MathEvaluateError, MathParseError};
use crate::calc_base::value::Value;
use crate::calc_strategies::ICalculatorStrategy;

/// Strategie určená jen pro testovací účely. Neprovádí žádné výpočty, jen vždy vrátí nulu.
#[derive(Default, Debug)]
pub struct ReturnAlwaysZeroStrategy<'a> {
    unused: PhantomData<&'a str>
}

impl<'expr> ICalculatorStrategy<'expr> for ReturnAlwaysZeroStrategy<'expr> {
    fn parse(&mut self, math_expr: &str) -> Result<(), MathParseError> {
        Ok(())
    }

    fn evaluate(&mut self) -> Result<Value, MathEvaluateError> {
        Ok(Value::Integer(0))
    }
}