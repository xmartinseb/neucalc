use std::{error::Error, fmt::Display};
use crate::base::IAppError;
use std::num::ParseFloatError;
use crate::define_error_type;
use num_bigint::*;

pub mod value;
mod rational;
pub mod func_call;
pub mod std_funcs;

/// Počítání celých čísel je na rozdíl od floatů přesné. Proto je lepší pracovat s tímto typem, kdykoliv to jde.
pub type Integer = i64;

/// Libovolně velké číslo je fajn, ole oproti normálnímu Integeru poněkud pomalé.
pub type BigInteger = num_bigint::BigInt;

/// Reálné číslo
pub type Real = f64;

// Chyba značící neplatně zadaný matematický výraz
define_error_type!(MathParseError);

// Chyba vzniklá při výpočtu výrazu
define_error_type!(MathEvaluateError);

impl From<ParseFloatError> for MathParseError {
    fn from(value: ParseFloatError) -> Self {
        Self::new_with_inner("Nepodařilo se převést text na číslo".into(), Box::new(value))
    }
}