pub mod common;
pub mod recursive_scan_strategy;
use crate::base::CalcError;
use crate::calc_base::expr::Expr;
use crate::calc_base::func_call::FuncCall;
use crate::calc_base::value::Value;

/// Strategie výpočtu textového výrazu.
/// Strategie určuje použitý algoritmus.
/// Generická struktura Calculator potřebuje dosadit typ strategie.
/// Algoritmus má vždy dva kroky, ale ne vždy musí být oba kroky implementovány.
/// (1) Parse převede textový výraz na nějakou logickou strukturu (např. na strom)
/// (2) Evaluate z logické struktury spočítá finální výsledek.
pub trait ICalculatorStrategy<'expr>: Default {
    fn parse(&mut self, math_expr: Expr<'expr>) -> Result<(), CalcError>;
    fn evaluate(&mut self) -> Result<Value, CalcError>;
    fn parse_func_call(&self, expr: Expr<'expr>) -> Result<FuncCall, CalcError>;
}
