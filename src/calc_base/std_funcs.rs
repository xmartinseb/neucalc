use crate::base::CalcError;
use crate::calc_base::rational::Rational;
use crate::calc_base::value::Value;
use crate::{rat, s};
use num_bigint::BigInt;
use num_traits::{FromPrimitive, One, Zero};
use num_traits::{Signed, ToPrimitive};
use std::cmp::Ordering;
use std::f64::consts::PI;

pub fn ln(x: Value) -> Result<Value, CalcError> {
    // let real = x.as_real()?;
    // Ok(Value::Real(real.ln()))
    match x {
        Value::Nothing => Err(CalcError::FuncCallErr(s!(
            "ln(Nothing) není platné volání funkce"
        ))),
        Value::Integer(i) => Ok(Value::Real((i as f64).ln())),
        Value::BigInt(i) => Ok(Value::Real(
            i.to_f64()
                .ok_or(CalcError::EvaluateErr(s!(
                    "ln: Nepodařilo se převést BigInt na reálné číslo"
                )))?
                .ln(),
        )),
        Value::Rational(q) => Ok(Value::Real(
            q.to_real()
                .ok_or(CalcError::EvaluateErr(s!(
                    "ln: Nepodařilo se převést zlomek  na reálné číslo"
                )))?
                .ln(),
        )),
        Value::Real(r) => Ok(Value::Real(r.ln())),
        Value::Text(_) => Err(CalcError::FuncCallErr(s!(
            "ln(Text) není platné volání funkce"
        ))),
        Value::Bool(_) => Err(CalcError::FuncCallErr(s!(
            "ln(Bool) není platné volání funkce"
        ))),
    }
}

pub fn abs(x: Value) -> Result<Value, CalcError> {
    match x {
        Value::Nothing => Err(CalcError::FuncCallErr(s!(
            "abs(Nothing) není platné volání funkce"
        ))),
        Value::Integer(i) => Ok(Value::Integer(i.abs())),
        Value::BigInt(i) => Ok(Value::BigInt(i.abs())),
        Value::Rational(q) => Ok(Value::Rational(q.abs())),
        Value::Real(r) => Ok(Value::Real(r.abs())),
        Value::Text(_) => Err(CalcError::FuncCallErr(s!(
            "abs(Text) není platné volání funkce"
        ))),
        Value::Bool(_) => Err(CalcError::FuncCallErr(s!(
            "abs(Bool) není platné volání funkce"
        ))),
    }
}

pub fn comb(n: Value, k: Value, repetition: Value) -> Result<Value, CalcError> {
    macro_rules! param_type_error {
        () => {
            Err(CalcError::FuncCallErr(s!(
                "Funkce comb(n,k,repetition) očekává dva celočíselné parametry a jeden bool"
            )))
        };
    }

    if let Value::Integer(n_int) = n {
        if let Value::Integer(k_int) = k {
            if let Value::Bool(rep_bool) = repetition {
                if n_int < 0 {
                    Err(CalcError::FuncCallErr(s!(
                        "Chyba: comb(n,k,repetition): n musí být >= 0"
                    )))
                } else if k_int < 0 {
                    Err(CalcError::FuncCallErr(s!(
                        "Chyba: comb(n,k,repetition): k musí být >= 0"
                    )))
                } else if n_int < k_int {
                    Err(CalcError::FuncCallErr(s!(
                        "Chyba: comb(n,k,repetition) vyžaduje: n >= k"
                    )))
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

pub fn comb_internal(n: i64, k: i64, repetition: bool) -> Result<Value, CalcError> {
    let result = if repetition {
        nck_internal(n + k - 1, k)
    } else {
        nck_internal(n, k)
    };

    match result {
        Ok(result_value) => Ok(result_value),
        Err(e) => Err(CalcError::EvaluateErr(format!(
            "Funkce comb využívá funkci nck, která skončila s chybou: {e}"
        ))),
    }
}

/// Kombinační číslo n nad k.
pub fn nck(n: Value, k: Value) -> Result<Value, CalcError> {
    macro_rules! param_type_error {
        () => {
            Err(CalcError::FuncCallErr(s!(
                "Funkce nck(n,k) očekává celočíselné parametry"
            )))
        };
    }
    if let Value::Integer(n_int) = n {
        if let Value::Integer(k_int) = k {
            nck_internal(n_int, k_int)
        } else {
            param_type_error!()
        }
    } else {
        param_type_error!()
    }
}

fn nck_internal(n: i64, k: i64) -> Result<Value, CalcError> {
    if n < 0 {
        Err(CalcError::FuncCallErr(s!(
            "Chyba: nck(n,k): n musí být >= 0"
        )))
    } else if k < 0 {
        Err(CalcError::FuncCallErr(s!(
            "Chyba: nck(n,k): k musí být >= 0"
        )))
    } else if n < k {
        Err(CalcError::FuncCallErr(s!(
            "Chyba: nck(n,k) vyžaduje: n >= k"
        )))
    } else if n == k || n == 0 {
        Ok(Value::Integer(1))
    } else {
        // Využijem toho, že n nad k == n nad (n-k). Zjednoduší to výpočty tím, že se budou
        // faktoriálovat menší čísla
        let n_min_k = n - k;
        let k = if n_min_k < k { n_min_k } else { k };

        let denominator = fact(Value::Integer(k)).map_err(|e| {
            CalcError::EvaluateErr(format!(
                "Funkce nck využívá faktoriál, který skončil s chybou: {e}"
            ))
        })?;

        let mut numerator = BigInt::from(n);
        for i in (n - k + 1)..n {
            numerator *= i;
        }
        (Value::BigInt(numerator) / denominator)?.simplify_type_move()
    }
}

pub fn fact(val: Value) -> Result<Value, CalcError> {
    const FACT_MAX: i64 = 100;

    match val {
        Value::Nothing => Err(CalcError::FuncCallErr(s!(
            "fact(Nothing) není platné volání funkce"
        ))),
        Value::Integer(n) => {
            return if n == 0 {
                Ok(Value::Integer(1))
            } else if n < 0 {
                Err(CalcError::FuncCallErr(format!(
                    "fact({n}) není platné volání funkce. Očekává se nezáporné číslo"
                )))
            } else if n <= FACT_MAX {
                let mut val = BigInt::from(n);
                for i in 2..n {
                    val *= i;
                }
                Value::BigInt(val).simplify_type_move()
            } else {
                Err(CalcError::FuncCallErr(format!(
                    "Nelze faktoriálovat čísla větší než {FACT_MAX}"
                )))
            }
        }
        Value::BigInt(n) => {
            return if let Some(int) = n.to_i64() {
                fact(Value::Integer(int))
            } else {
                Err(CalcError::FuncCallErr(s!("fact(BigInt) není platné volání funkce. Není povoleno faktoriálovat velká čísla")))
            }
        }
        Value::Rational(_) => Err(CalcError::FuncCallErr(s!(
            "fact(Rational) není platné volání funkce"
        ))),
        Value::Real(_) => Err(CalcError::FuncCallErr(s!(
            "fact(Real) není platné volání funkce"
        ))),
        Value::Text(_) => Err(CalcError::FuncCallErr(s!(
            "fact(Text) není platné volání funkce"
        ))),
        Value::Bool(_) => Err(CalcError::FuncCallErr(s!(
            "fact(Bool) není platné volání funkce"
        ))),
    }
}

/// Sinus - radiány
pub fn sin(rads: Value) -> Result<Value, CalcError> {
    // Radiány bývají iracionální číslo. Kromě hodnoty 0.
    match rads {
        Value::Nothing => Err(CalcError::FuncCallErr(s!(
            "sin(Nothing) není platné volání funkce"
        ))),
        Value::Integer(i) => {
            return if i == 0 {
                Ok(Value::Integer(0))
            } else {
                Ok(Value::Real((i as f64).sin()))
            }
        }
        Value::BigInt(i) => sin(Value::Integer((i % 360_i64).to_i64().unwrap())),
        Value::Rational(r) => Ok(Value::Real(
            r.to_real()
                .ok_or(CalcError::EvaluateErr(s!(
                    "Nepodařilo se zlomek převést na reálné číslo"
                )))?
                .sin(),
        )),
        Value::Real(r) => Ok(Value::Real(r.sin())),
        Value::Text(_) => Err(CalcError::FuncCallErr(s!(
            "sin(Text) není platné volání funkce"
        ))),
        Value::Bool(_) => Err(CalcError::FuncCallErr(s!(
            "sin(Bool) není platné volání funkce"
        ))),
    }
}

/// Sinus - radiány. Do parametru se ale nemusí dosazovat iracionální koeficient pí.
/// Pomáhá to udržet přesnost. Např. pi/2 se nedá reprezentovat přesně, ale 1/2 ano.
pub fn sinpi(rads_pi: Value) -> Result<Value, CalcError> {
    fn sinpi_err() -> CalcError {
        CalcError::FuncCallErr(
            "Funkce sinpi vyžaduje jako parametr: int, bigint, real, nebo rational".into(),
        )
    }

    match rads_pi {
        Value::Nothing => Err(sinpi_err()),
        Value::Integer(i) => sinpi_rat(Rational::new(i, 1)),
        Value::BigInt(bi) => sinpi_rat(Rational::from_bigint(bi)),
        Value::Rational(r) => sinpi_rat(r),
        Value::Real(re) => Ok(Value::Real((re * PI).sin())),
        Value::Text(_) => Err(sinpi_err()),
        Value::Bool(_) => Err(sinpi_err()),
    }
}

fn sinpi_rat(rat: Rational) -> Result<Value, CalcError> {
    todo!()
    // let rat = rat.reduce_move();

    // if rat == Rational::new(1, 6) {
    //     Ok(rat!(1 / 2))
    // } else if rat == Rational::new(1, 2) {
    //     Ok(Value::Integer(1))
    // } else if rat == Rational::new(3, 2) {
    //     Ok(Value::Integer(-1))
    // } else if rat.denominator == BigInt::one() {
    //     let a = rat.numerator % 2;
    //     if (a == BigInt::ZERO) {
    //         Ok(Value::Integer(1))
    //     } else if (a == BigInt::one()) {
    //     }
    // } else {
    //     let re = rat.to_real().ok_or(CalcError::ConvertToDoubleErr)?;
    //     Ok(Value::Real((re * PI).sin()))
    // }
}

/// Sinus - stupně
pub fn sind(degs: Value) -> Result<Value, CalcError> {
    match degs {
        Value::Nothing => Err(CalcError::FuncCallErr(s!(
            "sind(Nothing) není platné volání funkce"
        ))),
        Value::Integer(i) => Ok(sin_values_match_deg(i % 360)),
        Value::BigInt(i) => Ok(sin_values_match_deg((i % 360_i64).to_i64().unwrap())),
        Value::Rational(r) => Ok(Value::Real(
            r.to_real()
                .ok_or(CalcError::EvaluateErr(s!(
                    "Nepodařilo se zlomek převést na reálné číslo"
                )))?
                .to_radians()
                .sin(),
        )),
        Value::Real(r) => Ok(Value::Real(r.to_radians().sin())),
        Value::Text(_) => Err(CalcError::FuncCallErr(s!(
            "sind(Text) není platné volání funkce"
        ))),
        Value::Bool(_) => Err(CalcError::FuncCallErr(s!(
            "sind(Bool) není platné volání funkce"
        ))),
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
        _ => Value::Real((deg_0_360 as f64).to_radians().sin()),
    }
}

fn cos_values_match_deg(deg_0_360: i64) -> Value {
    match deg_0_360 {
        // TODO: Doplnit odmocniny ze dvou, tří. Az se zavede novy typ value
        0 => Value::Integer(1),
        60 => rat!(1 / 2),
        90 => Value::Integer(0),
        120 => rat!(-1 / 2),
        180 => Value::Integer(-1),
        240 => rat!(-1 / 2),
        270 => Value::Integer(0),
        300 => rat!(1 / 2),
        _ => Value::Real((deg_0_360 as f64).to_radians().sin()),
    }
}

pub fn max(params: &Vec<Value>) -> Result<Value, CalcError> {
    // TODO: Bude implementováno, až se budou dát hodnoty porovnávat!
    // Ok(Value::Integer(params.len() as i64))
    todo!()
}

pub fn sqrt(val: Value) -> Result<Value, CalcError> {
    return match val {
        Value::Nothing => Err(CalcError::FuncCallErr(s!(
            "sqrt(Nothing) není platné volání funkce"
        ))),
        Value::Integer(input_i) => {
            let i_real = input_i as f64;
            let result_real = i_real.sqrt();
            let result_int = result_real as i64;
            return if result_int.pow(2u32) == input_i {
                Ok(Value::Integer(result_int))
            } else {
                Ok(Value::Real(result_real))
            };
        }
        Value::BigInt(input_i) => {
            let i_real = input_i.to_f64().ok_or(CalcError::EvaluateErr(s!(
                "Nepodařilo se odmocnit velké celé číslo"
            )))?;
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
            return num / den; // Může znovu vzniknout zlomek, nebo i celé nebo reálné číslo
        }
        Value::Real(r) => Ok(Value::Real(r.sqrt())),
        Value::Text(_) => Err(CalcError::FuncCallErr(s!(
            "sqrt(Text) není platné volání funkce"
        ))),
        Value::Bool(_) => Err(CalcError::FuncCallErr(s!(
            "sqrt(Bool) není platné volání funkce"
        ))),
    };
}

pub fn cista_mzda(hruba: Value) -> Result<Value, CalcError> {
    match hruba {
        Value::Nothing => Err(CalcError::FuncCallErr(s!(
            "cista(Nothing) není platné volání funkce"
        ))),
        Value::Integer(int) => Ok(Value::Rational(cista_mzda_impl(Rational::new(int, 1))?)),
        Value::BigInt(big_int) => Ok(Value::Rational(cista_mzda_impl(Rational {
            numerator: big_int,
            denominator: BigInt::one(),
        })?)),
        Value::Rational(rational) => Ok(Value::Rational(cista_mzda_impl(rational)?)),
        Value::Real(_) => Err(CalcError::FuncCallErr(s!(
            "cista(Real) není platné volání funkce"
        ))),
        Value::Text(_) => Err(CalcError::FuncCallErr(s!(
            "cista(Text) není platné volání funkce"
        ))),
        Value::Bool(_) => Err(CalcError::FuncCallErr(s!(
            "cista(Bool) není platné volání funkce"
        ))),
    }
}

fn cista_mzda_impl(hruba: Rational) -> Result<Rational, CalcError> {
    if hruba.is_negative() || hruba.numerator == BigInt::zero() {
        return Err(CalcError::FuncCallErr(s!(
            "Funkce 'cista' nemůže spočítat čistou mzdu ze záporného čísla ani z nuly"
        )));
    }

    // sazby pojistného
    let r0075 = Rational::new(75, 1000);
    let r0045 = Rational::new(45, 1000);
    let r015 = Rational::new(15, 100);
    let r023 = Rational::new(23, 100);

    let sp = r0075 * hruba.clone(); // sociální
    let zp = r0045 * hruba.clone(); // zdravotní

    // daňový základ = hrubá mzda zaokrouhlená na stovky nahoru
    let dz =
        ((hruba.to_real().expect("Selhal převod hodnoty na f64") / 100.0).ceil() * 100.0) as i64;
    let dz = Rational::new(dz, 1);

    // hranice pro 23% daň (měsíčně, 2025 ~161 000 Kč)
    let hranice_23 = Rational::new(161000, 1);

    // daň před slevami
    let dan: Rational;
    if compare_positive_rationals(hruba.clone(), hranice_23.clone()) != Ordering::Greater {
        dan = r015 * dz;
    } else {
        dan = r015 * hranice_23.clone() + r023 * (dz - hranice_23.clone());
    }

    // sleva na poplatníka (měsíční)
    let sleva = Rational::new(2570, 1);
    let mut dan_po_sleve = dan - sleva;
    if dan_po_sleve.is_negative() {
        dan_po_sleve = Rational::zero();
    }

    // čistá mzda
    Ok(hruba - sp - zp - dan_po_sleve)
}

fn compare_positive_rationals(a: Rational, b: Rational) -> Ordering {
    assert!(a.numerator > Default::default());
    assert!(b.numerator > Default::default());
    assert!(a.denominator > Default::default());
    assert!(b.denominator > Default::default());

    let left = a.numerator * b.denominator;
    let right = b.numerator * a.denominator;
    return left.cmp(&right);
}
