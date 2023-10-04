use std::marker::PhantomData;
use crate::base::IAppError;
use crate::calc_base::value::Value;
use crate::calc_strategies::common::check_brackets_and_quots;
use crate::calc_strategies::ICalculatorStrategy;

/// Calculator pomocí metody evaluate_expr vypočítá zadaný matematický výraz. Potřebuje ale
/// doplnit typ strategie. Strategie určuje použitý algoritmus parsování a výpočtů.
pub struct Calculator<'expr, TStrategy: ICalculatorStrategy<'expr>> {
    g : PhantomData<TStrategy>, // Phantom data nic neobsahuje, jen vyznačuje kompilátoru, jak se používají generické parametry
    h : PhantomData<&'expr str>
}

impl<'expr, TStrategy: ICalculatorStrategy<'expr>> Calculator<'expr, TStrategy> {
    pub fn evaluate_expr(&self, math_expr: &'expr str) -> Result<Value, Box<dyn IAppError>> {
        check_brackets_and_quots(math_expr)?;

        let mut calc_strategy : TStrategy = Default::default();
        return match calc_strategy.parse(math_expr) {
            Ok(_) => {
                match calc_strategy.evaluate() {
                    Ok(value) => Ok(value),
                    Err(e) => Err(Box::new(e)),
                }
            },
            Err(parse_err) => { Err(Box::new(parse_err)) }
        }
    }

    pub fn new() -> Self {
        return Self{
            g: PhantomData::default(),
            h: PhantomData::default(),
        };
    }
}

impl<'expr, TStagegy: ICalculatorStrategy<'expr>> Default for Calculator<'expr, TStagegy> {
    fn default() -> Self {
        Calculator::new()
    }
}
