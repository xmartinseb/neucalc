use std::{error::Error, fmt::Display};
use crate::base::IAppError;
use std::num::ParseFloatError;
use crate::define_error_type;
use num_bigint::*;

pub mod value;
pub mod rational;
pub mod func_call;
pub mod std_funcs;
pub mod value_algebra;
pub mod expr;

// Chyba značící neplatně zadaný matematický výraz
define_error_type!(MathParseError);

// Chyba vzniklá při výpočtu výrazu
define_error_type!(MathEvaluateError);

impl From<ParseFloatError> for MathParseError {
    fn from(value: ParseFloatError) -> Self {
        Self::new_with_inner("Nepodařilo se převést text na číslo".into(), Box::new(value))
    }
}