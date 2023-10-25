use num_bigint::BigInt;
use num_integer::Roots;
use num_traits::{Signed, ToPrimitive};
use num_traits::FromPrimitive;
use crate::calc_base::MathEvaluateError;
use crate::calc_base::rational::Rational;
use crate::calc_base::value::Value;
use crate::s;

pub fn abs(x: Value) -> Result<Value, MathEvaluateError> {
    match x {
        Value::Nothing => Err(MathEvaluateError::new(s!("abs(Nothing) není platné volání funkce"))),
        Value::Integer(i) => Ok(Value::Integer(i.abs())),
        Value::BigInt(i) => Ok(Value::BigInt(i.abs())),
        Value::Rational(q) => Ok(Value::Rational(q.abs())),
        Value::Real(r) => Ok(Value::Real(r.abs())),
        Value::Text(_) => Err(MathEvaluateError::new(s!("abs(Text) není platné volání funkce"))),
        Value::Bool(_) => Err(MathEvaluateError::new(s!("abs(Bool) není platné volání funkce")))
    }
}

pub fn comb(n: Value, k: Value, repetition: Value) -> Result<Value, MathEvaluateError> {
    macro_rules! param_type_error {
        () => {Err(MathEvaluateError::new(s!("Funkce comb(n,k,repetition) očekává dva celočíselné parametry a jeden bool")))};
    }

    if let Value::Integer(n_int) = n
    {
        if let Value::Integer(k_int) = k {
            if let Value::Bool(rep_bool) = repetition {
                if n_int < 0 {
                    Err(MathEvaluateError::new(s!("Chyba: comb(n,k,repetition): n musí být >= 0")))
                } else if k_int < 0 {
                    Err(MathEvaluateError::new(s!("Chyba: comb(n,k,repetition): k musí být >= 0")))
                } else if n_int < k_int {
                    Err(MathEvaluateError::new(s!("Chyba: comb(n,k,repetition) vyžaduje: n >= k")))
                } else {
                    comb_internal(n_int, k_int, rep_bool)
                }
            } else {
                param_type_error!()
            }
        } else {
            param_type_error!()
        }
    } else {
        param_type_error!()
    }
}

pub fn comb_internal(n: i64, k: i64, repetition: bool) -> Result<Value, MathEvaluateError> {
    let result = if repetition {
        nck_internal(n+k-1, k)
    } else {
        nck_internal(n, k)
    };

    match result {
        Ok(result_value) => {Ok(result_value)}
        Err(e) => {
            Err(MathEvaluateError::new(format!("Funkce comb využívá funkci nck, která skončila s chybou: {e}")))
        }
    }
}

/// Kombinační číslo n nad k.
pub fn nck(n: Value, k: Value) -> Result<Value, MathEvaluateError> {
    macro_rules! param_type_error {
        () => {Err(MathEvaluateError::new(s!("Funkce nck(n,k) očekává celočíselné parametry")))};
    }
    if let Value::Integer(n_int) = n
    {
        if let Value::Integer(k_int) = k {
            nck_internal(n_int, k_int)
        } else {
            param_type_error!()
        }
    } else {
        param_type_error!()
    }
}

fn nck_internal(n: i64, k: i64) -> Result<Value, MathEvaluateError>{

    if n < 0 {
        Err(MathEvaluateError::new(s!("Chyba: nck(n,k): n musí být >= 0")))
    } else if k < 0 {
        Err(MathEvaluateError::new(s!("Chyba: nck(n,k): k musí být >= 0")))
    } else if n < k {
        Err(MathEvaluateError::new(s!("Chyba: nck(n,k) vyžaduje: n >= k")))
    } else if n == k || n == 0 {
        Ok(Value::Integer(1))
    } else {
        // Využijem toho, že n nad k == n nad (n-k). Zjednoduší to výpočty tím, že se budou
        // faktoriálovat menší čísla
        let n_min_k = n-k;
        let k = if n_min_k < k {
            n_min_k
        } else {k};

        let denominator = fact(Value::Integer(k))
            .map_err(|e| MathEvaluateError::new(format!("Funkce nck využívá faktoriál, který skončil s chybou: {e}")))?;

        let mut numerator = BigInt::from(n);
        for i in (n-k+1)..n {
            numerator *= i;
        }
        (Value::BigInt(numerator) / denominator)?.simplify_type_move()
    }
}

pub fn fact(val: Value) -> Result<Value, MathEvaluateError> {
    const FACT_MAX: i64 = 100;

    match val {
        Value::Nothing =>  Err(MathEvaluateError::new(s!("fact(Nothing) není platné volání funkce"))),
        Value::Integer(n) => {
            return if n == 0 {
                Ok(Value::Integer(1))
            } else if n < 0 {
                Err(MathEvaluateError::new(format!("fact({n}) není platné volání funkce. Očekává se nezáporné číslo")))
            } else if n <= FACT_MAX {
                let mut val = BigInt::from(n);
                for i in 2..n {
                    val *= i;
                }
                Value::BigInt(val).simplify_type_move()
            } else {
                Err(MathEvaluateError::new(format!("Nelze faktoriálovat čísla větší než {FACT_MAX}")))
            }
        }
        Value::BigInt(n) => {
            return if let Some(int) = n.to_i64() {
                fact(Value::Integer(int))
            } else {
                Err(MathEvaluateError::new(s!("fact(BigInt) není platné volání funkce. Není povoleno faktoriálovat velká čísla")))
            }
        }
        Value::Rational(_) => Err(MathEvaluateError::new(s!("fact(Rational) není platné volání funkce"))),
        Value::Real(_) => Err(MathEvaluateError::new(s!("fact(Real) není platné volání funkce"))),
        Value::Text(_) => Err(MathEvaluateError::new(s!("fact(Text) není platné volání funkce"))),
        Value::Bool(_) => Err(MathEvaluateError::new(s!("fact(Bool) není platné volání funkce"))),
    }
}

/// Sinus - radiány
pub fn sin(rads: Value) -> Result<Value, MathEvaluateError> {
    // Radiány bývají iracionální číslo. Kromě hodnoty 0.
    match rads {
        Value::Nothing => Err(MathEvaluateError::new(s!("sin(Nothing) není platné volání funkce"))),
        Value::Integer(i) => {
            return if i == 0 {
                Ok(Value::Integer(0))
            } else {
                Ok(Value::Real((i as f64).sin()))
            }
        }
        Value::BigInt(i) => sin(Value::Integer((i % 360_i64).to_i64().unwrap())),
        Value::Rational(r) => Ok(Value::Real(r.to_real()
            .ok_or(MathEvaluateError::new(s!("Nepodařilo se zlomek převést na reálné číslo")))?.sin())),
        Value::Real(r) => {Ok(Value::Real(r.sin()))}
        Value::Text(_) => {Err(MathEvaluateError::new(s!("sin(Text) není platné volání funkce")))}
        Value::Bool(_) => {Err(MathEvaluateError::new(s!("sin(Bool) není platné volání funkce")))}
    }
}

/// Sinus - radiány. Do parametru se ale nemusí dosazovat iracionální koeficient pí.
/// Pomáhá to udržet přesnost. Např. pi/2 se nedá reprezentovat přesně, ale 1/2 ano.
pub fn sinpi(rads_pi: Value) -> Result<Value, MathEvaluateError> {
    todo!()
}

/// Sinus - stupně
pub fn sind(degs: Value) -> Result<Value, MathEvaluateError> {
    match degs {
        Value::Nothing => Err(MathEvaluateError::new(s!("sind(Nothing) není platné volání funkce"))),
        Value::Integer(i) => Ok(sin_values_match_deg(i % 360)),
        Value::BigInt(i) => Ok(sin_values_match_deg((i % 360_i64).to_i64().unwrap())),
        Value::Rational(r) => Ok(Value::Real(r.to_real()
            .ok_or(MathEvaluateError::new(s!("Nepodařilo se zlomek převést na reálné číslo")))?.to_radians().sin())),
        Value::Real(r) => {Ok(Value::Real(r.to_radians().sin()))}
        Value::Text(_) => {Err(MathEvaluateError::new(s!("sind(Text) není platné volání funkce")))}
        Value::Bool(_) => {Err(MathEvaluateError::new(s!("sind(Bool) není platné volání funkce")))}
    }
}

fn sin_values_match_deg(deg_0_360: i64) -> Value {
    match deg_0_360 {
        // TODO: Doplnit odmocniny ze dvou, tří. Az se zavede novy typ value
        0 => Value::Integer(0),
        30 => Value::Rational(Rational::new(1, 2)),
        90 => Value::Integer(1),
        150 => Value::Rational(Rational::new(1, 2)),
        180 => Value::Integer(0),
        210 => Value::Rational(Rational::new(-1, 2)),
        270 => Value::Integer(-1),
        330 => Value::Rational(Rational::new(-1, 2)),
        _ => Value::Real((deg_0_360 as f64).to_radians().sin())
    }
}


fn cos_values_match_deg(deg_0_360: i64) -> Value {
    match deg_0_360 {
        // TODO: Doplnit odmocniny ze dvou, tří. Az se zavede novy typ value
        0 => Value::Integer(1),
        60 => Value::Rational(Rational::new(1, 2)),
        90 => Value::Integer(0),
        120 => Value::Rational(Rational::new(-1, 2)),
        180 => Value::Integer(-1),
        240 => Value::Rational(Rational::new(-1, 2)),
        270 => Value::Integer(0),
        300 => Value::Rational(Rational::new(1, 2)),
        _ => Value::Real((deg_0_360 as f64).to_radians().sin())
    }
}

pub fn max(params: &Vec<Value>) -> Result<Value, MathEvaluateError> {
    // TODO: Bude implementováno, až se budou dát hodnoty porovnávat!
    Ok(Value::Integer(params.len() as i64))
}

pub fn sqrt(val: Value) -> Result<Value, MathEvaluateError> {
    return  match val {
        Value::Nothing => Err(MathEvaluateError::new(s!("sqrt(Nothing) není platné volání funkce"))),
        Value::Integer(input_i) => {
            let i_real = input_i as f64;
            let result_real = i_real.sqrt();
            let result_int = result_real as i64;
            return if result_int.pow(2u32) == input_i {
                Ok(Value::Integer(result_int))
            } else {
                Ok(Value::Real(result_real))
            }
        }
        Value::BigInt(input_i) => {
            let i_real = input_i.to_f64()
                .ok_or(MathEvaluateError::new(s!("Nepodařilo se odmocnit velké celé číslo")))?;
            let result_real = i_real.sqrt();
            let result_int = BigInt::from_f64(result_real);

            if let Some(res_int) = result_int {
                if res_int.pow(2u32) == input_i {
                    return Ok(Value::BigInt(res_int));
                }
            }
            return Ok(Value::Real(result_real));
        }
        Value::Rational(rat) => {
            let num = sqrt(Value::BigInt(rat.numerator.clone()))?;
            let den = sqrt(Value::BigInt(rat.denominator.clone()))?;
            return  num / den; // Může znovu vzniknout zlomek, nebo i celé nebo reálné číslo
        },
        Value::Real(r) => Ok(Value::Real(r.sqrt())),
        Value::Text(_) => Err(MathEvaluateError::new(s!("sqrt(Text) není platné volání funkce"))),
        Value::Bool(_) => Err(MathEvaluateError::new(s!("sqrt(Bool) není platné volání funkce"))),
    }
}