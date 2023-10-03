use std::{error::Error, fmt::Display};
use crate::base::IAppError;
use std::num::ParseFloatError;
use crate::define_error_type;


pub mod value;

/// Počítání celých čísel je na rozdíl od floatů přesné. Proto je lepší pracovat s tímto typem, kdykoliv to jde.
pub type Integer = i64;

/// Příští verze možná zavede zlomky (tj. dvě celá čísla, které se dělí). Prozatím tento typ neexistuje.
pub type Rational = f64;

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