pub mod recursive_scan_strategy;
mod always_zero_strategy;
pub mod common;

use crate::calc_base::{MathEvaluateError, MathParseError};
use crate::calc_base::value::Value;

/// Strategie výpočtu textového výrazu.
/// Strategie určuje použitý algoritmus.
/// Generická struktura Calculator potřebuje dosadit typ strategie.
/// Algoritmus má vždy dva kroky, ale ne vždy musí být oba kroky implementovány.
/// (1) Parse převede textový výraz na nějakou logickou strukturu (např. na strom)
/// (2) Evaluate z logické struktury spočítá finální výsledek.
pub trait ICalculatorStrategy<'expr> : Default
{
    fn parse(&mut self, math_expr: &'expr str) -> Result<(), MathParseError>;
    fn evaluate(&mut self) -> Result<Value, MathEvaluateError>;
}