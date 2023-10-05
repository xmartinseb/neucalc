use std::fmt::Display;
use crate::calc_base::*;
use crate::calc_base::rational::Rational;
use num_traits::cast::ToPrimitive;
use crate::calc_base::func_call::FuncCall;
use crate::calc_strategies::ICalculatorStrategy;
use crate::s;

/// Hodnota, se kterou se pracuje při výpočtu matematického výrazu, může mít různé typy.
/// Jsou na ní definovány matematické operace +,-,*,/, pow
#[derive(Debug, Clone)]
pub enum Value {
    Nothing,
    Integer(i64),
    BigInt(BigInt),
    Rational(Rational),
    Real(f64),
    Text(String),
    Bool(bool),
    Func(FuncCall),
}

/// Platné konstanty
pub mod consts {
    use crate::calc_base::value::Value;

    pub static PI: Value = Value::Real(std::f64::consts::PI);
    pub static E: Value = Value::Real(std::f64::consts::E);
    pub static SQRT2: Value = Value::Real(std::f64::consts::SQRT_2);
    pub static SQRT3: Value = Value::Real(1.73205080756887729352744634150587236694280525381038);
    pub static I64MAX: Value = Value::Integer(i64::MAX);
    pub static I64MIN: Value = Value::Integer(i64::MIN);
}

mod system_consts {
    use lazy_static::lazy_static;
    use num_bigint::BigInt;

    lazy_static! {
        pub static ref BI_64MAX: BigInt = BigInt::from(i64::MAX);
        pub static ref BI_64MIN: BigInt = BigInt::from(i64::MIN);
    }
}

/// Pokud je name platný název konstanty, vrátí se její hodnota, jinak se vrátí None
fn is_named_const(name: &str) -> Option<Value> {
    return match name.trim().to_lowercase().as_str() {
        "pi" => Some(consts::PI.clone()),
        "e" => Some(consts::E.clone()),
        "sqrt2" => Some(consts::SQRT2.clone()),
        "sqrt3" => Some(consts::SQRT3.clone()),
        "i64max" => Some(consts::I64MAX.clone()),
        "i64min" => Some(consts::I64MIN.clone()),
        _ => None
    };
}

fn value_is_string_literal(expr: &str) -> Option<&str> {
    let expr = expr.trim();
    let mut is_in_string = false;

    let len = expr.chars().count();
    return if len >= 2 && expr.starts_with('"') && expr.ends_with('"') {
        for (idx, c) in expr.chars().enumerate() {
            if c == '"' {
                is_in_string = !is_in_string;
            }

            if idx < len - 1 && !is_in_string {
                return None;
            }
        }
        Some(&expr[1..expr.len() - 1])
    } else {
        None
    }
}

impl Value {
    pub fn parse<'expr>(value: &str, strategy: &impl ICalculatorStrategy<'expr>) -> Result<Self, MathEvaluateError> {
        let value = value.trim();
        if let Some(string_value) = value_is_string_literal(value) {
            return Ok(Value::Text(s!(string_value)))
        } else if let Ok(integer) = value.parse::<i64>() {
            return Ok(Value::Integer(integer));
        } else if let Ok(biginteger) = value.parse::<BigInt>() {
            return Ok(Value::BigInt(biginteger).simplify_type_move()?);
        } else if let Ok(ratio) = value.parse::<Rational>() {
            return Ok(Value::Rational(ratio).simplify_type_move()?);
        } else if let Some(val_const) = is_named_const(value) {
            return Ok(val_const);
        } else if let Ok(boolean) = value.parse::<bool>() {
            return Ok(Value::Bool(boolean));
        } else if let Ok(func_call) = strategy.parse_func_call(value) {
            return Ok(Value::Func(func_call));
        } else if let Ok(real) = value.parse::<f64>() { //Reálná čísla by neměla být parsovatelná z konzole
            println!("Varování! Vstup z konzole se načetl jako reálné číslo. Zpravidla se načítá \
            racionální číslo. Výsledek nemusí být úplně přesný!");
            return Ok(Value::Real(real));
        }
        Err(MathEvaluateError::new(format!("Výraz '{value}' není platná hodnota.")))
    }

    // V některých případech lze považovat BigInteger za Integer. Někdy je zase zlomek
    // celým číslem. Tato metoda najde co nejjednodušší typ.
    pub fn simplify_type_move(self) -> Result<Self, MathEvaluateError> {
        let val = match self {
            Value::Nothing => self,
            Value::Integer(_) => self,
            Value::Text(_) => self,
            Value::Func(f) => f.eval()?,
            Value::Bool(_) => self,
            Value::Real(r) => if r == 0.0 {Value::Integer(0)} else {self},
            Value::BigInt(b) => {
                if let Some(i) = b.to_i64() {
                    Value::Integer(i)
                } else {
                    Value::BigInt(b)
                }
            }
            Value::Rational(r) => {
                let r = r.reduce_move();
                if let Some(as_bigint) = r.to_bigint() {
                    if let Some(as_int) = as_bigint.to_i64() {
                        Value::Integer(as_int)
                    } else {
                        Value::BigInt(as_bigint)
                    }
                } else { Value::Rational(r) }
            }
        };

        Ok(val)
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Nothing
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nothing => write!(f, "{}", "{}"),
            Value::Integer(x) => write!(f, "{x}\t(integer)"),
            Value::BigInt(x) => write!(f, "{x}\t(big integer)"),
            Value::Rational(x) => write!(f, "{x}\t(rational)"),
            Value::Real(x) => write!(f, "{x}\t(real)"),
            Value::Text(x) => write!(f, "\"{x}\""),
            Value::Bool(x) => write!(f, "{x}"),
            Value::Func(func) => write!(f, "{}({})", func.name(), func.params_as_string()),
        }
    }
}

