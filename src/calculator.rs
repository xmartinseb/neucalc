use crate::base::CalcError;
use crate::calc_base::expr::Expr;
use crate::calc_base::value::Value;
use crate::calc_strategies::common::check_brackets_and_quots;
use crate::calc_strategies::ICalculatorStrategy;
use std::marker::PhantomData;

/// Calculator pomocí metody evaluate_expr vypočítá zadaný matematický výraz. Potřebuje ale
/// doplnit typ strategie. Strategie určuje použitý algoritmus parsování a výpočtů.
pub struct Calculator<'expr, TStrategy: ICalculatorStrategy<'expr>> {
    g: PhantomData<TStrategy>, // Phantom data nic neobsahuje, jen vyznačuje kompilátoru, jak se používají generické parametry
    h: PhantomData<&'expr str>,
}

impl<'expr, TStrategy: ICalculatorStrategy<'expr>> Calculator<'expr, TStrategy> {
    pub fn evaluate_expr(&self, math_expr: &'expr str) -> Result<Value, CalcError> {
        check_brackets_and_quots(math_expr)?;

        // Výraz prošel validační procedurou, nyní je považován za syntakticky správný
        let mut calc_strategy: TStrategy = Default::default();
        return match calc_strategy.parse(Expr::new(math_expr)) {
            // 1. krok strategie: parse
            Ok(_) => {
                // 2. krok strategie: evaluace parsovaneho vyrazu
                calc_strategy.evaluate()
            }
            Err(parse_err) => Err(parse_err),
        };
    }
}

impl<'expr, TStagegy: ICalculatorStrategy<'expr>> Default for Calculator<'expr, TStagegy> {
    fn default() -> Self {
        Calculator {
            g: Default::default(),
            h: Default::default(),
        }
    }
}
