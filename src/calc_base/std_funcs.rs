use num_bigint::BigInt;
use num_integer::Roots;
use num_traits::ToPrimitive;
use num_traits::FromPrimitive;
use crate::calc_base::MathEvaluateError;
use crate::calc_base::rational::Rational;
use crate::calc_base::value::Value;
use crate::s;

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
            println!("TEST: {:?} / {:?}", num, den);
            return  num / den; // Může znovu vzniknout zlomek, nebo i celé nebo reálné číslo
        },
        Value::Real(r) => Ok(Value::Real(r.sqrt())),
        Value::Text(_) => Err(MathEvaluateError::new(s!("sqrt(Text) není platné volání funkce"))),
        Value::Bool(_) => Err(MathEvaluateError::new(s!("sqrt(Bool) není platné volání funkce"))),
    }
}