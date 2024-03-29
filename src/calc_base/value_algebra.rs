use crate::base::CalcError;
use crate::calc_base::rational::Rational;
use crate::calc_base::value::Value;
use crate::calc_base::*;
use crate::s;
use num_traits::cast::ToPrimitive;
use num_traits::Zero;
use std::ops::{Add, Mul, Neg, Sub};

impl Neg for Value {
    type Output = Result<Value, CalcError>;

    fn neg(self) -> Self::Output {
        return match self {
            Value::Nothing => Ok(Value::Nothing),
            Value::Integer(x) => {
                return if let Some(result) = x.checked_neg() {
                    Ok(Value::Integer(result))
                } else {
                    Ok(Value::BigInt(-BigInt::from(x)))
                }
            }
            Value::BigInt(x) => Ok(Value::BigInt(-x)),
            Value::Rational(x) => Ok(Value::Rational(-x)),
            Value::Real(x) => Ok(Value::Real(-x)),
            Value::Text(x) => Err(CalcError::EvaluateErr(format!(
                "Na text {x} nelze aplikovat unární mínus"
            ))),
            Value::Bool(x) => Err(CalcError::EvaluateErr(format!(
                "Na boolean {x} nelze aplikovat unární mínus"
            ))),
        };
    }
}

fn simplify_result_type(x: Result<Value, CalcError>) -> Result<Value, CalcError> {
    x?.simplify_type_move()
}

impl Sub<Value> for Value {
    type Output = Result<Value, CalcError>;

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
                }
                Value::BigInt(y) => Ok(Value::BigInt(BigInt::from(x) - y)),
                Value::Rational(y) => Ok(Value::Rational(Rational::from_int(x) - y)),
                Value::Real(y) => Ok(Value::Real(x as f64 - y)),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze odčítat celé číslo {x} a text {y}"
                ))),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze odčítat celé číslo {x} a boolean {y}"
                ))),
            },
            Value::BigInt(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::BigInt(x - BigInt::from(y))),
                Value::BigInt(y) => Ok(Value::BigInt(x - y)),
                Value::Rational(y) => Ok(Value::Rational(Rational::from_bigint(x) - y)),
                Value::Real(y) => Ok(Value::Real(bi_to_real(&x)? - y)),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze odčítat velké celé číslo {x} a text {y}"
                ))),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze odčítat velké celé číslo {x} a boolean {y}"
                ))),
            },
            Value::Rational(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Rational(x - Rational::from_int(y))),
                Value::BigInt(y) => Ok(Value::Rational(x - Rational::from_bigint(y))),
                Value::Rational(y) => Ok(Value::Rational(x - y)),
                Value::Real(y) => Ok(Value::Real(to_real(&x)? - y)),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze odčítat racionální číslo {x} a text {y}"
                ))),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze odčítat racionální číslo {x} a boolean {y}"
                ))),
            },
            Value::Real(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Real(x - y as f64)),
                Value::BigInt(y) => Ok(Value::Real(x - bi_to_real(&y)?)),
                Value::Rational(y) => Ok(Value::Real(x - to_real(&y)?)),
                Value::Real(y) => Ok(Value::Real(x - y)),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze odčítat reálné číslo {x} a text {y}"
                ))),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze odčítat reálné číslo {x} a boolean {y}"
                ))),
            },
            Value::Text(x) => Err(CalcError::EvaluateErr(format!(
                "Na text {x} nelze aplikovat operátor minus"
            ))),
            Value::Bool(x) => Err(CalcError::EvaluateErr(format!(
                "Na boolean {x} nelze aplikovat operátor minus"
            ))),
        };
        return simplify_result_type(result);
    }
}

fn to_real(q: &Rational) -> Result<f64, CalcError> {
    return match q.to_real() {
        None => Err(CalcError::EvaluateErr(format!(
            "Zlomek '{q}' se nepodařilo převést na reálné číslo"
        ))),
        Some(r) => Ok(r),
    };
}

fn bi_to_real(q: &BigInt) -> Result<f64, CalcError> {
    return match q.to_f64() {
        None => Err(CalcError::EvaluateErr(format!(
            "Velké celé číslo '{q}' se nepodařilo převést na reálné číslo"
        ))),
        Some(r) => Ok(r),
    };
}

impl std::ops::Div<Value> for Value {
    type Output = Result<Value, CalcError>;

    fn div(self, rhs: Value) -> Self::Output {
        // Kontrola dělení nulou aj.
        match &rhs {
            Value::Nothing => {}
            Value::Integer(i) => {
                if i.is_zero() {
                    return Err(CalcError::EvaluateErr(s!("Nelze dělit celým číslem 0")));
                }
            }
            Value::BigInt(i) => {
                if i.is_zero() {
                    return Err(CalcError::EvaluateErr(s!("Nelze dělit bigintem 0")));
                }
            }
            Value::Rational(r) => {
                if r.numerator.is_zero() {
                    return Err(CalcError::EvaluateErr(s!(
                        "Nelze dělit racionálním číslem 0"
                    )));
                }
            }
            Value::Real(r) => {
                if r.is_zero() {
                    return Err(CalcError::EvaluateErr(s!("Nelze dělit reálným číslem 0")));
                }
            }
            Value::Text(_) => {
                return Err(CalcError::EvaluateErr(s!("Nelze dělit textovou hodnotu.")));
            }
            Value::Bool(_) => {
                return Err(CalcError::EvaluateErr(s!(
                    "Nelze dělit booleovskou bodnotu"
                )));
            }
        };

        let result = match self {
            Value::Nothing => Ok(Value::Nothing),
            Value::Integer(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(div_ints(x, y)?), // Dělení celých čísel může vrátit zlomek, nebo i celé číslo!
                Value::BigInt(y) => Ok(div_big_ints(&BigInt::from(x), &y)?), // Dělení celých čísel může vrátit zlomek, nebo i celé číslo!
                Value::Rational(y) => Ok(Value::Rational(Rational::from_int(x) / y)),
                Value::Real(y) => Ok(Value::Real(x as f64 / y)),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze dělit celé číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze dělit celé číslo {x} a boolean {y}."
                ))),
            },
            Value::BigInt(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(div_big_ints(&x, &BigInt::from(y))?), // Dělení celých čísel může vrátit zlomek, nebo i celé číslo!
                Value::BigInt(y) => Ok(div_big_ints(&x, &y)?), // Dělení celých čísel může vrátit zlomek, nebo i celé číslo!
                Value::Rational(y) => Ok(Value::Rational(Rational::from_bigint(x) / y)),
                Value::Real(y) => Ok(Value::Real(bi_to_real(&x)? / y)),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze dělit velké celé číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze dělit velké celé číslo {x} a boolean {y}."
                ))),
            },
            Value::Rational(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Rational(x / Rational::from_int(y))),
                Value::BigInt(y) => Ok(Value::Rational(x / Rational::from_bigint(y))),
                Value::Rational(y) => Ok(Value::Rational(x / y)),
                Value::Real(y) => Ok(Value::Real(to_real(&x)? / y)),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze dělit racionální číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze dělit racionální číslo {x} a boolean {y}."
                ))),
            },
            Value::Real(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Real(x / y as f64)),
                Value::BigInt(y) => Ok(Value::Real(x / bi_to_real(&y)?)),
                Value::Rational(y) => Ok(Value::Real(x / to_real(&y)?)),
                Value::Real(y) => Ok(Value::Real(x / y)),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze dělit reálné číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze dělit reálné číslo {x} a boolean {y}."
                ))),
            },
            Value::Text(x) => Err(CalcError::EvaluateErr(format!(
                "Na text {x} neze aplikovat operátor dělení"
            ))),
            Value::Bool(_) => Err(CalcError::EvaluateErr(format!(
                "Na boolean neze aplikovat operátor dělení"
            ))),
        };
        return simplify_result_type(result);
    }
}

/// Dělení dvou celých čísel může vrátit zlomek (racio. číslo), nebo celé číslo
fn div_ints(a: i64, b: i64) -> Result<Value, CalcError> {
    if b == 0 {
        Err(CalcError::EvaluateErr(s!("Nulou nelze dělit!")))
    } else {
        Ok(Value::Rational(Rational::new(a, b))
            .simplify_type_move()
            .unwrap())
    }
}

/// Dělení dvou celých čísel může vrátit zlomek (racio. číslo), nebo celé číslo
fn div_big_ints(a: &BigInt, b: &BigInt) -> Result<Value, CalcError> {
    if b.is_zero() {
        Err(CalcError::EvaluateErr(s!("Nulou nelze dělit!")))
    } else {
        Ok(Value::Rational(Rational::new_bigint(a.clone(), b.clone()))
            .simplify_type_move()
            .unwrap())
    }
}

impl Mul<Value> for Value {
    type Output = Result<Value, CalcError>;

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
                }
                Value::BigInt(y) => Ok(Value::BigInt(x * y)),
                Value::Rational(y) => Ok(Value::Rational(Rational::from_int(x) * y)),
                Value::Real(y) => Ok(Value::Real(x as f64 * y)),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze násobit celé číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze násobit celé číslo {x} a boolean {y}."
                ))),
            },
            Value::BigInt(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::BigInt(x * y)),
                Value::BigInt(y) => Ok(Value::BigInt(x * y)),
                Value::Rational(y) => Ok(Value::Rational(Rational::from_bigint(x) * y)),
                Value::Real(y) => Ok(Value::Real(bi_to_real(&x)? * y)),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze násobit velké celé číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze násobit velké celé číslo {x} a boolean {y}."
                ))),
            },
            Value::Rational(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Rational(x * Rational::from_int(y))),
                Value::BigInt(y) => Ok(Value::Rational(x * Rational::from_bigint(y))),
                Value::Rational(y) => Ok(Value::Rational(x * y)),
                Value::Real(y) => Ok(Value::Real(to_real(&x)? * y)),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze násobit racionální číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze násobit racionální číslo {x} a boolean {y}."
                ))),
            },
            Value::Real(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Real(x * y as f64)),
                Value::BigInt(y) => Ok(Value::Real(x * bi_to_real(&y)?)),
                Value::Rational(y) => Ok(Value::Real(x * to_real(&y)?)),
                Value::Real(y) => Ok(Value::Real(x * y)),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze násobit reálné číslo {x} a text {y}."
                ))),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze násobit reálné číslo {x} a boolean {y}."
                ))),
            },
            Value::Text(x) => Err(CalcError::EvaluateErr(format!(
                "Na text {x} neze aplikovat operátor násobení"
            ))),
            Value::Bool(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze násobit bool {x} a celé číslo {y}."
                ))),
                Value::BigInt(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze násobit bool {x} a velké celé číslo {y}."
                ))),
                Value::Rational(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze násobit bool {x} a racionální číslo {y}."
                ))),
                Value::Real(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze násobit bool {x} a reálné číslo {y}."
                ))),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze násobit bool {x} a text {y}."
                ))),
                Value::Bool(y) => Ok(Value::Bool(x && y)),
            },
        };
        return simplify_result_type(result);
    }
}

impl Add<Value> for Value {
    type Output = Result<Value, CalcError>;

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
                }
                Value::BigInt(y) => Ok(Value::BigInt(x + y)),
                Value::Rational(y) => Ok(Value::Rational(Rational::from_int(x) + y)),
                Value::Real(y) => Ok(Value::Real(x as f64 + y)),
                Value::Text(y) => Ok(Value::Text(x.to_string() + &y)),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
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
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
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
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze sčítat racionální číslo {x} a boolean {y}."
                ))),
            },
            Value::Real(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Real(x + y as f64)),
                Value::BigInt(y) => Ok(Value::Real(x + bi_to_real(&y)?)),
                Value::Rational(y) => Ok(Value::Real(x + to_real(&y)?)),
                Value::Real(y) => Ok(Value::Real(x + y)),
                Value::Text(y) => Ok(Value::Text(x.to_string() + &y)),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
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
                Value::Integer(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze sčítat bool {x} a celé číslo {y}."
                ))),
                Value::BigInt(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze sčítat bool {x} a velké celé číslo {y}."
                ))),
                Value::Rational(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze sčítat bool {x} a racionální číslo {y}."
                ))),
                Value::Real(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze sčítat bool {x} a reálné číslo {y}."
                ))),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze sčítat bool {x} a text {y}."
                ))),
                Value::Bool(y) => Ok(Value::Bool(x || y)),
            },
        };
        return simplify_result_type(result);
    }
}

impl Value {
    pub fn pow(self, rhs: &Value) -> Result<Value, CalcError> {
        match self {
            Value::Nothing => Ok(Value::Nothing),
            Value::Integer(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => {
                    if *y >= 0 {
                        let res = match x.checked_pow(*y as u32) {
                            None => Value::BigInt(BigInt::from(x).pow(*y as u32)),
                            Some(r) => Value::Integer(r),
                        };
                        Ok(res)
                    } else if *y == 0 {
                        return Ok(Value::Integer(1));
                    } else {
                        let res = match x.checked_pow((-*y) as u32) {
                            None => {
                                (Value::Integer(1)
                                    / Value::BigInt(BigInt::from(x).pow((-*y) as u32)))?
                            }
                            Some(r) => (Value::Integer(1) / Value::Integer(r))?,
                        };
                        Ok(res)
                    }
                }
                Value::BigInt(_) => Err(CalcError::EvaluateErr(s!(
                    "Mocnění velkých celých čísel není povoleno"
                ))),
                Value::Rational(y) => Ok(Value::Real((x as f64).powf(to_real(y)?))),
                Value::Real(y) => Ok(Value::Real((x as f64).powf(*y))),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze mocnit celé číslo {x} na text {y}."
                ))),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze mocnit celé číslo {x} na boolean {y}."
                ))),
            },
            Value::BigInt(_) => Err(CalcError::EvaluateErr(s!(
                "Mocnění velkých celých čísel není povoleno"
            ))),
            Value::Rational(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Rational(x.pow_int(*y))),
                Value::BigInt(_) => Err(CalcError::EvaluateErr(s!(
                    "Mocnění velkých celých čísel není povoleno"
                ))),
                Value::Rational(y) => Ok(Value::Real(to_real(&x)?.powf(to_real(y)?))),
                Value::Real(y) => Ok(Value::Real(to_real(&x)?.powf(*y))),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze mocnit celé číslo {x} na text {y}."
                ))),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze mocnit celé číslo {x} na boolean {y}."
                ))),
            },
            Value::Real(x) => match rhs {
                Value::Nothing => Ok(Value::Nothing),
                Value::Integer(y) => Ok(Value::Real(x.powf(*y as f64))),
                Value::BigInt(_) => Err(CalcError::EvaluateErr(s!(
                    "Mocnění velkých celých čísel není povoleno"
                ))),
                Value::Rational(y) => Ok(Value::Real(x.powf(to_real(y)?))),
                Value::Real(y) => Ok(Value::Real(x.powf(*y))),
                Value::Text(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze mocnit celé číslo {x} na text {y}."
                ))),
                Value::Bool(y) => Err(CalcError::EvaluateErr(format!(
                    "Nelze mocnit celé číslo {x} na boolean {y}."
                ))),
            },
            Value::Text(x) => Err(CalcError::EvaluateErr(format!("Nelze mocnit text {x}."))),
            Value::Bool(x) => Err(CalcError::EvaluateErr(format!("Nelze mocnit boolean {x}."))),
        }
    }
}
