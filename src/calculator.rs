use std::marker::PhantomData;
use crate::base::IAppError;
use crate::calc_base::value::Value;
use crate::calc_strategies::ICalculatorStrategy;

/// Calculator pomocí metody evaluate_expr vypočítá zadaný matematický výraz. Potřebuje ale
/// doplnit typ strategie. Strategie určuje použitý algoritmus parsování a výpočtů.
pub struct Calculator<'a, TStrategy: ICalculatorStrategy<'a>> {
    g : PhantomData<TStrategy>, // Phantom data nic neobsahuje, jen vyznačuje kompilátoru, jak se používají generické parametry
    h : PhantomData<&'a str>
}

impl<'a, TStrategy: ICalculatorStrategy<'a>> Calculator<'a, TStrategy> {
    pub fn evaluate_expr(&self, math_expr: &'a str) -> Result<Value, Box<dyn IAppError>> {
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

impl<'a, TStagegy: ICalculatorStrategy<'a>> Default for Calculator<'a, TStagegy> {
    fn default() -> Self {
        Calculator::new()
    }
}
