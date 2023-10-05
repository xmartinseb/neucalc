use crate::base::IAppError;
use crate::calc_base::{MathEvaluateError, MathParseError};
use crate::calc_base::func_call::FuncCall;
use crate::calc_base::value::Value;
use crate::calc_strategies::ICalculatorStrategy;

/// Strategie určená jen pro testovací účely. Neprovádí žádné výpočty, jen vždy vrátí nulu.
#[derive(Default, Debug)]
pub struct AlwaysZeroStrategy<'expr> {
    math_str: &'expr str
}

impl<'expr> ICalculatorStrategy<'expr> for AlwaysZeroStrategy<'expr> {
    fn parse(&mut self, math_expr: &'expr str) -> Result<(), MathParseError> {
        self.math_str = math_expr;
        Ok(())
    }

    fn evaluate(&mut self) -> Result<Value, MathEvaluateError> {
        Ok(Value::Integer(0))
    }

    fn parse_func_call(&self, _: &str) -> Result<FuncCall, Box<dyn IAppError>> {
        todo!()
    }
}