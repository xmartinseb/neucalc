use std::fmt::Display;
use std::ops::{Add, Sub};
use crate::calc_base::*;
use crate::calc_base::rational::Rational;
use num_traits::cast::ToPrimitive;
use crate::s;

/// Hodnota, se kterou se pracuje při výpočtu matematického výrazu, může mít různé typy.
/// Jsou na ní definovány matematické operace +,-,*,/, pow
#[derive(Debug, Clone)]
pub enum Value {
    Nothing,
    Integer(Integer),
    BigInt(BigInteger),
    Rational(Rational),
    Real(Real),
    Text(String),
    Bool(bool),
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
    use crate::calc_base::BigInteger;

    lazy_static! {
        pub static ref BI_64MAX: BigInteger = BigInteger::from(i64::MAX);
        pub static ref BI_64MIN: BigInteger = BigInteger::from(i64::MIN);
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
    let len = expr.len();
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
    pub fn parse(value: &str) -> Result<Self, MathEvaluateError> {
        let value = value.trim();
        if let Some(string_value) = value_is_string_literal(value) {
            return Ok(Value::Text(s!(string_value)))
        }
        else if let Some(val_const) = is_named_const(value){
            return Ok(val_const);
        }
        else if let Ok(boolean) = value.parse::<bool>() {
            return Ok(Value::Bool(boolean));
        }else if let Ok(integer) = value.parse::<Integer>() {
            return Ok(Value::Integer(integer));
        } else if let Ok(biginteger) = value.parse::<BigInteger>() {
            return Ok(Value::BigInt(biginteger));
        } else if let Ok(real) = value.parse::<Real>() {
            return Ok(Value::Real(real));
        }
        Err(MathEvaluateError::new(format!("Výraz '{value}' není platná hodnota.")))
    }

    // V některých případech lze považovat BigInteger za Integer. Někdy je zase zlomek
    // celým číslem. Tato metoda najde co nejjednodušší typ.
    pub fn simplify_type_move(self) -> Self {
        return match self {
            Value::Nothing => self,
            Value::Integer(_) => self,
            Value::Text(_) => self,
            Value::Bool(_) => self,
            Value::Real(r) => return if r == 0.0 {Value::Integer(0)} else {self},
            Value::BigInt(b) => {
                if let Some(i) = b.to_i64() {
                    Value::Integer(i)
                } else {
                    Value::BigInt(b)
                }
            }
            Value::Rational(r) => {
                return if let Some(as_bigint) = r.to_bigint() {
                    if let Some(as_int) = as_bigint.to_i64() {
                        Value::Integer(as_int)
                    } else {
                        Value::BigInt(as_bigint)
                    }
                } else { Value::Rational(r) }
            }
        };
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
        }
    }
}

impl std::ops::Neg for Value {
    type Output = Result<Value, MathEvaluateError>;

    fn neg(self) -> Self::Output {
        return match self {
            Value::Nothing => Ok(Value::Nothing),
            Value::Integer(x) => {
                return if let Some(result) = x.checked_neg() {
                    Ok(Value::Integer(result))
                } else {
                    Ok(Value::BigInt(-BigInt::from(x)))
                }
            },
            Value::BigInt(x) => Ok(Value::BigInt(-x)),
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

fn simplify_result_type(x: Result<Value, MathEvaluateError>) -> Result<Value, MathEvaluateError> {
    Ok(x?.simplify_type_move())
}

impl Sub<Value> for Value {
    type Output = Result<Value, MathEvaluateError>;

    fn sub(self, rhs: Value) -> Self::Output {
        let result = match self {
            Value::Nothing => Ok(Value::Nothing),
            Value::Integer(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => {
                    return if let Some(result) = x.checked_sub(y) {
                        Ok(Value::Integer(result))
                    } else {
                        Ok(Value::BigInt(BigInt::from(x) - y))
                    }
                },
                Value::BigInt(y) => Ok(Value::BigInt(BigInt::from(x) - y)),
                Value::Rational(y) => Ok(Value::Rational(Rational::from_int(x) - y)),
                Value::Real(y) => Ok(Value::Real(x as Real - y)),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze odčítat celé číslo {x} a text {y}"
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze odčítat celé číslo {x} a boolean {y}"
                ))),
            },
            Value::BigInt(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::BigInt(x - BigInteger::from(y))),
                Value::BigInt(y) => Ok(Value::BigInt(x - y)),
                Value::Rational(y) => Ok(Value::Rational(Rational::from_bigint(x) - y)),
                Value::Real(y) => Ok(Value::Real(bi_to_real(&x)? - y)),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze odčítat velké celé číslo {x} a text {y}"
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze odčítat velké celé číslo {x} a boolean {y}"
                ))),
            },
            Value::Rational(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Rational(x - Rational::from_int(y))),
                Value::BigInt(y) => Ok(Value::Rational(x - Rational::from_bigint(y))),
                Value::Rational(y) => Ok(Value::Rational(x - y)),
                Value::Real(y) => Ok(Value::Real(to_real(&x)?  - y)),
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
                Value::BigInt(y) => Ok(Value::Real(x - bi_to_real(&y)? )),
                Value::Rational(y) => Ok(Value::Real(x - to_real(&y)? )),
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
        };
        return simplify_result_type(result);
    }
}

fn to_real(q: &Rational) -> Result<Real, MathEvaluateError> {
    return match q.to_real() {
        None => {Err(MathEvaluateError::new(format!("Zlomek '{q}' se nepodařilo převést na reálné číslo")))}
        Some(r) => {Ok(r)}
    }
}

fn bi_to_real(q: &BigInteger) -> Result<Real, MathEvaluateError> {
    return match q.to_f64() {
        None => {Err(MathEvaluateError::new(format!("Velké celé číslo '{q}' se nepodařilo převést na reálné číslo")))}
        Some(r) => {Ok(r)}
    }
}

impl std::ops::Div<Value> for Value {
    type Output = Result<Value, MathEvaluateError>;

    fn div(self, rhs: Value) -> Self::Output {
        let result = match self {
            Value::Nothing => Ok(Value::Nothing),
            Value::Integer(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(div_ints(x, y)), // Dělení celých čísel může vrátit zlomek, nebo i celé číslo!
                Value::BigInt(y) => Ok(div_big_ints(&BigInteger::from(x), &y)), // Dělení celých čísel může vrátit zlomek, nebo i celé číslo!
                Value::Rational(y) => Ok(Value::Rational(Rational::from_int(x) / y)),
                Value::Real(y) => Ok(Value::Real(x as Real / y)),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze dělit celé číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze dělit celé číslo {x} a boolean {y}."
                ))),
            },
            Value::BigInt(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(div_big_ints(&x, &BigInt::from(y))), // Dělení celých čísel může vrátit zlomek, nebo i celé číslo!
                Value::BigInt(y) => Ok(div_big_ints(&x, &y)), // Dělení celých čísel může vrátit zlomek, nebo i celé číslo!
                Value::Rational(y) => Ok(Value::Rational(Rational::from_bigint(x) / y)),
                Value::Real(y) => Ok(Value::Real(bi_to_real(&x)?  / y)),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze dělit velké celé číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze dělit velké celé číslo {x} a boolean {y}."
                ))),
            },
            Value::Rational(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Rational(x / Rational::from_int(y))),
                Value::BigInt(y) => Ok(Value::Rational(x / Rational::from_bigint(y))),
                Value::Rational(y) => Ok(Value::Rational(x / y)),
                Value::Real(y) => Ok(Value::Real(to_real(&x)? / y)),
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
                Value::BigInt(y) => Ok(Value::Real(x / bi_to_real(&y)?)),
                Value::Rational(y) => Ok(Value::Real(x / to_real(&y)?)),
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
                Value::BigInt(y) => Err(MathEvaluateError::new(format!(
                    "Nelze dělit bool {x} a velké celé číslo {y}."
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
        };
        return simplify_result_type(result);
    }
}

/// Dělení dvou celých čísel může vrátit zlomek (racio. číslo), nebo celé číslo
fn div_ints(a: Integer, b: Integer) -> Value {
    let ri = a / b;
    let ri_asr = ri as f64;
    let rr = a as f64 / b as f64;

    return if (ri_asr - rr).abs() < 0.0000001 {
        Value::Integer(ri)
    } else {
        Value::Rational(Rational::new(a, b).reduce_move())
    }
}

/// Dělení dvou celých čísel může vrátit zlomek (racio. číslo), nebo celé číslo
fn div_big_ints(a: &BigInteger, b: &BigInteger) -> Value {
    let ri = a / b;
    let ri_asr = bi_to_real(&ri).ok();
    let a_asr = bi_to_real(&a).ok();
    let b_asr = bi_to_real(&b).ok();

    return if ri_asr.is_some() && a_asr.is_some() && b_asr.is_some() {
        let rr = a_asr.unwrap() / b_asr.unwrap();

        if (ri_asr.unwrap() - rr).abs() < 0.0000001 {
            Value::BigInt(ri)
        } else {
            Value::Rational(Rational::new_bigint(a.clone(), b.clone()))
        }
    } else {
        Value::Rational(Rational::new_bigint(a.clone(), b.clone()))
    }
}

impl std::ops::Mul<Value> for Value {
    type Output = Result<Value, MathEvaluateError>;

    fn mul(self, rhs: Value) -> Self::Output {
        let result = match self {
            Value::Nothing => Ok(Value::Nothing),
            Value::Integer(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => {
                    return if let Some(result) = x.checked_mul(y) {
                        Ok(Value::Integer(result))
                    } else {
                        Ok(Value::BigInt(BigInt::from(x) * y))
                    }
                },
                Value::BigInt(y) => Ok(Value::BigInt(x * y)),
                Value::Rational(y) => Ok(Value::Rational(Rational::from_int(x) * y)),
                Value::Real(y) => Ok(Value::Real(x as Real * y)),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze násobit celé číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze násobit celé číslo {x} a boolean {y}."
                ))),
            },
            Value::BigInt(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::BigInt(x * y)),
                Value::BigInt(y) => Ok(Value::BigInt(x * y)),
                Value::Rational(y) => Ok(Value::Rational(Rational::from_bigint(x) * y)),
                Value::Real(y) => Ok(Value::Real(bi_to_real(&x)?  * y)),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze násobit velké celé číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze násobit velké celé číslo {x} a boolean {y}."
                ))),
            },
            Value::Rational(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Rational(x * Rational::from_int(y))),
                Value::BigInt(y) => Ok(Value::Rational(x * Rational::from_bigint(y))),
                Value::Rational(y) => Ok(Value::Rational(x * y)),
                Value::Real(y) => Ok(Value::Real(to_real(&x)?  * y)),
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
                Value::BigInt(y) => Ok(Value::Real(x * bi_to_real(&y)?)),
                Value::Rational(y) => Ok(Value::Real(x * to_real(&y)?)),
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
                Value::BigInt(y) => Err(MathEvaluateError::new(format!(
                    "Nelze násobit bool {x} a velké celé číslo {y}."
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
        };
        return simplify_result_type(result);
    }
}

impl Add<Value> for Value {
    type Output = Result<Value, MathEvaluateError>;

    fn add(self, rhs: Value) -> Self::Output {
        let result = match self {
            Value::Nothing => Ok(Value::Nothing),
            Value::Integer(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => {
                    return if let Some(result) = x.checked_add(y) {
                        Ok(Value::Integer(result))
                    } else {
                        Ok(Value::BigInt(BigInt::from(x) + y))
                    }
                },
                Value::BigInt(y) => Ok(Value::BigInt(x + y)),
                Value::Rational(y) => Ok(Value::Rational(Rational::from_int(x) + y)),
                Value::Real(y) => Ok(Value::Real(x as Real + y)),
                Value::Text(y) => Ok(Value::Text(x.to_string() + &y)),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze sčítat celé číslo {x} a boolean {y}."
                ))),
            },
            Value::BigInt(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::BigInt(x + y)),
                Value::BigInt(y) => Ok(Value::BigInt(x + y)),
                Value::Rational(y) => Ok(Value::Rational(Rational::from_bigint(x) + y)),
                Value::Real(y) => Ok(Value::Real(bi_to_real(&x)? + y)),
                Value::Text(y) => Ok(Value::Text(x.to_string() + &y)),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze sčítat velké celé číslo {x} a boolean {y}."
                ))),
            },
            Value::Rational(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Rational(x + Rational::from_int(y))),
                Value::BigInt(y) => Ok(Value::Rational(x + Rational::from_bigint(y))),
                Value::Rational(y) => Ok(Value::Rational(x + y)),
                Value::Real(y) => Ok(Value::Real(to_real(&x)? + y)),
                Value::Text(y) => Ok(Value::Text(x.to_string() + &y)),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze sčítat racionální číslo {x} a boolean {y}."
                ))),
            },
            Value::Real(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Real(x + y as Real)),
                Value::BigInt(y) => Ok(Value::Real(x + bi_to_real(&y)?)),
                Value::Rational(y) => Ok(Value::Real(x + to_real(&y)?)),
                Value::Real(y) => Ok(Value::Real(x + y)),
                Value::Text(y) => Ok(Value::Text(x.to_string() + &y)),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze sčítat reálné číslo {x} a boolean {y}."
                ))),
            },
            Value::Text(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Text(x + &y.to_string())),
                Value::BigInt(y) => Ok(Value::Text(x + &y.to_string())),
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
                Value::BigInt(y) => Err(MathEvaluateError::new(format!(
                    "Nelze sčítat bool {x} a velké celé číslo {y}."
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
        };
        return simplify_result_type(result);
    }
}

impl Value {
    pub fn pow(self, rhs: &Value) -> Result<Value, MathEvaluateError> {
        match self {
            Value::Nothing => Ok(Value::Nothing),
            Value::Integer(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => {
                    if *y >= 0 {
                        let res = match x.checked_pow(*y as u32) {
                            None => Value::BigInt(BigInt::from(x).pow(*y as u32)),
                            Some(r) => Value::Integer(r)
                        };
                        Ok(res)
                    } else if *y == 0 {
                        return Ok(Value::Integer(1));
                    } else {
                        let res = match x.checked_pow((-*y) as u32) {
                            None => (Value::Integer(1) / Value::BigInt(BigInt::from(x).pow((-*y) as u32)))?,
                            Some(r) => (Value::Integer(1) / Value::Integer(r))?
                        };
                        Ok(res)
                    }
                },
                Value::BigInt(_) => Err(MathEvaluateError::new(s!("Mocnění velkých celých čísel není povoleno"))),
                Value::Rational(y) => Ok(Value::Real((x as Real).powf( to_real(y)?))),
                Value::Real(y) => Ok(Value::Real((x as Real).powf(*y))),
                Value::Text(y) => Err(MathEvaluateError::new(format!(
                    "Nelze mocnit celé číslo {x} na text {y}."
                ))),
                Value::Bool(y) => Err(MathEvaluateError::new(format!(
                    "Nelze mocnit celé číslo {x} na boolean {y}."
                ))),
            },
            Value::BigInt(_) => Err(MathEvaluateError::new(s!("Mocnění velkých celých čísel není povoleno"))),
            Value::Rational(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Real(to_real(&x)?.powf(*y as f64))), // TODO: Rational pote upravit: Q^Z € Q
                Value::BigInt(_) => Err(MathEvaluateError::new(s!("Mocnění velkých celých čísel není povoleno"))),
                Value::Rational(y) => Ok(Value::Real(to_real(&x)?.powf( to_real(y)?))),
                Value::Real(y) => Ok(Value::Real(to_real(&x)?.powf(*y))),
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
                Value::BigInt(_) => Err(MathEvaluateError::new(s!("Mocnění velkých celých čísel není povoleno"))),
                Value::Rational(y) => Ok(Value::Real(x.powf(to_real(y)?))),
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
