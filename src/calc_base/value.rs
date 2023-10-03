use std::fmt::Display;
use std::ops::{Add, Sub};
use crate::calc_base::*;

/// Hodnota, se kterou se pracuje při výpočtu matematického výrazu, může mít různé typy.
/// Jsou na ní definovány matematické operace +,-,*,/, pow
#[derive(Debug, Clone)]
pub enum Value {
    Nothing,
    Integer(Integer),
    Rational(Rational),
    Real(Real),
    Text(String),
    Bool(bool),
}

impl Value {
    pub fn parse(value: &str) -> Result<Self, MathEvaluateError> {
        let value = value.trim();
        if let Ok(boolean) = value.parse::<bool>() {
            return Ok(Value::Bool(boolean));
        }else if let Ok(integer) = value.parse::<Integer>() {
            return Ok(Value::Integer(integer));
        } else if let Ok(real) = value.parse::<Real>() {
            return Ok(Value::Real(real));
        } else if let Ok(rational) = value.parse::<Rational>() {
            return Ok(Value::Rational(rational));
        }
        Err(MathEvaluateError::new(format!("Výraz '{value}' není platná hodnota.")))
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
            Value::Rational(x) => write!(f, "{x}\t(rational)"),
            Value::Real(x) => write!(f, "{x}\t(real)"),
            Value::Text(x) => write!(f, "\"{x}\""),
            Value::Bool(x) => write!(f, "{x}"),
        }
    }
}

impl std::ops::Neg for Value {
    type Output = Result<Value, MathEvaluateError>;

    fn neg(self) -> Self::Output {
        return match self {
            Value::Nothing => Ok(Value::Nothing),
            Value::Integer(x) => Ok(Value::Integer(-x)),
            Value::Rational(x) => Ok(Value::Rational(-x)),
            Value::Real(x) => Ok(Value::Real(-x)),
            Value::Text(x) => Err(MathEvaluateError::new(format!(
                "Na text {x} nelze aplikovat unární mínus"
            ))),
            Value::Bool(x) => Err(MathEvaluateError::new(format!(
                "Na boolean {x} nelze aplikovat unární mínus"
            ))),
        };
    }
}

impl Sub<Value> for Value {
    type Output = Result<Value, MathEvaluateError>;

    fn sub(self, rhs: Value) -> Self::Output {
        match self {
            Value::Nothing => Ok(Value::Nothing),
            Value::Integer(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Integer(x - y)),
                Value::Rational(y) => Ok(Value::Rational(x as Rational - y)),
                Value::Real(y) => Ok(Value::Real(x as Real - y)),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze odčítat celé číslo {x} a text {y}"
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze odčítat celé číslo {x} a boolean {y}"
                ))),
            },
            Value::Rational(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Rational(x - y as Rational)),
                Value::Rational(y) => Ok(Value::Rational(x - y)),
                Value::Real(y) => Ok(Value::Real(x as Real - y)),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze odčítat racionální číslo {x} a text {y}"
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze odčítat racionální číslo {x} a boolean {y}"
                ))),
            },
            Value::Real(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Real(x - y as Real)),
                Value::Rational(y) => Ok(Value::Real(x - y as Real)),
                Value::Real(y) => Ok(Value::Real(x - y)),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze odčítat reálné číslo {x} a text {y}"
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze odčítat reálné číslo {x} a boolean {y}"
                ))),
            },
            Value::Text(x) => Err(MathEvaluateError::new(format!(
                "Na text {x} nelze aplikovat operátor minus"
            ))),
            Value::Bool(x) => Err(MathEvaluateError::new(format!(
                "Na boolean {x} nelze aplikovat operátor minus"
            ))),
        }
    }
}

impl std::ops::Div<Value> for Value {
    type Output = Result<Value, MathEvaluateError>;

    fn div(self, rhs: Value) -> Self::Output {
        match self {
            Value::Nothing => Ok(Value::Nothing),
            Value::Integer(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Rational(x as Rational / y as Rational)),
                Value::Rational(y) => Ok(Value::Rational(x as Rational / y)),
                Value::Real(y) => Ok(Value::Real(x as Real / y)),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze dělit celé číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze dělit celé číslo {x} a boolean {y}."
                ))),
            },
            Value::Rational(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Rational(x / y as Rational)),
                Value::Rational(y) => Ok(Value::Rational(x / y)),
                Value::Real(y) => Ok(Value::Real(x as Real / y)),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze dělit racionální číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze dělit racionální číslo {x} a boolean {y}."
                ))),
            },
            Value::Real(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Real(x / y as Real)),
                Value::Rational(y) => Ok(Value::Real(x / y as Real)),
                Value::Real(y) => Ok(Value::Real(x / y)),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze dělit reálné číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze dělit reálné číslo {x} a boolean {y}."
                ))),
            },
            Value::Text(x) => Err(MathEvaluateError::new(format!(
                "Na text {x} neze aplikovat operátor dělení"
            ))),
            Value::Bool(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Err(MathEvaluateError::new(format!(
                    "Nelze dělit bool {x} a celé číslo {y}."
                ))),
                Value::Rational(y) => Err(MathEvaluateError::new(format!(
                    "Nelze dělit bool {x} a racionální číslo {y}."
                ))),
                Value::Real(y) => Err(MathEvaluateError::new(format!(
                    "Nelze dělit bool {x} a reálné číslo {y}."
                ))),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze dělit bool {x} a text {y}."
                ))),
                Value::Bool(y) => Ok(Value::Bool(x && y)),
            },
        }
    }
}

impl std::ops::Mul<Value> for Value {
    type Output = Result<Value, MathEvaluateError>;

    fn mul(self, rhs: Value) -> Self::Output {
        match self {
            Value::Nothing => Ok(Value::Nothing),
            Value::Integer(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Integer(x * y)),
                Value::Rational(y) => Ok(Value::Rational(x as Rational * y)),
                Value::Real(y) => Ok(Value::Real(x as Real * y)),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze násobit celé číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze násobit celé číslo {x} a boolean {y}."
                ))),
            },
            Value::Rational(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Rational(x * y as Rational)),
                Value::Rational(y) => Ok(Value::Rational(x * y)),
                Value::Real(y) => Ok(Value::Real(x as Real * y)),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze násobit racionální číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze násobit racionální číslo {x} a boolean {y}."
                ))),
            },
            Value::Real(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Real(x * y as Real)),
                Value::Rational(y) => Ok(Value::Real(x * y as Real)),
                Value::Real(y) => Ok(Value::Real(x * y)),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze násobit reálné číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze násobit reálné číslo {x} a boolean {y}."
                ))),
            },
            Value::Text(x) => Err(MathEvaluateError::new(format!(
                "Na text {x} neze aplikovat operátor násobení"
            ))),
            Value::Bool(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Err(MathEvaluateError::new(format!(
                    "Nelze násobit bool {x} a celé číslo {y}."
                ))),
                Value::Rational(y) => Err(MathEvaluateError::new(format!(
                    "Nelze násobit bool {x} a racionální číslo {y}."
                ))),
                Value::Real(y) => Err(MathEvaluateError::new(format!(
                    "Nelze násobit bool {x} a reálné číslo {y}."
                ))),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze násobit bool {x} a text {y}."
                ))),
                Value::Bool(y) => Ok(Value::Bool(x && y)),
            },
        }
    }
}

impl Add<Value> for Value {
    type Output = Result<Value, MathEvaluateError>;

    fn add(self, rhs: Value) -> Self::Output {
        match self {
            Value::Nothing => Ok(Value::Nothing),
            Value::Integer(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Integer(x + y)),
                Value::Rational(y) => Ok(Value::Rational(x as Rational + y)),
                Value::Real(y) => Ok(Value::Real(x as Real + y)),
                Value::Text(y) => Ok(Value::Text(x.to_string() + &y)),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze sčítat celé číslo {x} a boolean {y}."
                ))),
            },
            Value::Rational(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Rational(x + y as Rational)),
                Value::Rational(y) => Ok(Value::Rational(x + y)),
                Value::Real(y) => Ok(Value::Real(x as Real + y)),
                Value::Text(y) => Ok(Value::Text(x.to_string() + &y)),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze sčítat racionální číslo {x} a boolean {y}."
                ))),
            },
            Value::Real(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Real(x + y as Real)),
                Value::Rational(y) => Ok(Value::Real(x + y as Real)),
                Value::Real(y) => Ok(Value::Real(x + y)),
                Value::Text(y) => Ok(Value::Text(x.to_string() + &y)),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze sčítat reálné číslo {x} a boolean {y}."
                ))),
            },
            Value::Text(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Text(x + &y.to_string())),
                Value::Rational(y) => Ok(Value::Text(x + &y.to_string())),
                Value::Real(y) => Ok(Value::Text(x + &y.to_string())),
                Value::Text(y) => Ok(Value::Text(x + &y)),
                Value::Bool(y) => Ok(Value::Text(x + &y.to_string())),
            },
            Value::Bool(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Err(MathEvaluateError::new(format!(
                    "Nelze sčítat bool {x} a celé číslo {y}."
                ))),
                Value::Rational(y) => Err(MathEvaluateError::new(format!(
                    "Nelze sčítat bool {x} a racionální číslo {y}."
                ))),
                Value::Real(y) => Err(MathEvaluateError::new(format!(
                    "Nelze sčítat bool {x} a reálné číslo {y}."
                ))),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze sčítat bool {x} a text {y}."
                ))),
                Value::Bool(y) => Ok(Value::Bool(x || y)),
            },
        }
    }
}

impl Value {
    pub fn pow(self, rhs: &Value) -> Result<Value, MathEvaluateError> {
        // // Vnitrni pomocna funkce
        // fn powf_or_powi<TNum>(val: &Value, exponent: TNum) -> Result<Value, MathEvaluateError>{
        //     todo!()
        // }

        match self {
            Value::Nothing => Ok(Value::Nothing),
            Value::Integer(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => {
                    if *y >= 0 {
                        Ok(Value::Integer(x.pow(*y as u32)))
                    } else if *y == 0 {
                        return Ok(Value::Integer(1));
                    } else {
                        let res = (Value::Integer(1) / Value::Integer(x.pow((-*y) as u32)))?;
                        Ok(res)
                    }
                }
                Value::Rational(y) => Ok(Value::Real((x as Real).powf(*y))),
                Value::Real(y) => Ok(Value::Real((x as Real).powf(*y))),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze mocnit celé číslo {x} na text {y}."
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze mocnit celé číslo {x} na boolean {y}."
                ))),
            },
            Value::Rational(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Rational(x.powf(*y as f64))), // TODO: Rational pote upravit: Q^Z € Q
                Value::Rational(y) => Ok(Value::Real((x as Real).powf(*y))),
                Value::Real(y) => Ok(Value::Real((x as Real).powf(*y))),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze mocnit celé číslo {x} na text {y}."
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze mocnit celé číslo {x} na boolean {y}."
                ))),
            },
            Value::Real(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Real(x.powf(*y as Real))),
                Value::Rational(y) => Ok(Value::Real(x.powf(*y))),
                Value::Real(y) => Ok(Value::Real(x.powf(*y))),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze mocnit celé číslo {x} na text {y}."
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze mocnit celé číslo {x} na boolean {y}."
                ))),
            },
            Value::Text(x) => Err(MathEvaluateError::new(format!(
                "Nelze mocnit text {x}."
            ))),
            Value::Bool(x) => Err(MathEvaluateError::new(format!(
                "Nelze mocnit boolean {x}."
            ))),
        }
    }
}
