use num_bigint::BigInt;
use std::str::FromStr;

/// Makro převede něco na strukturu String
#[macro_export]
macro_rules! s {
    ($str:expr) => {
        String::from($str)
    };
}

/// Flushne výstupní konzoli, čímž se vytiskne vše, co je v bufferu.
#[macro_export]
macro_rules! flush_stdout {
    () => {
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    };
}

/// Simple print with one or multiple parameters. Flushes IO automatically
#[macro_export]
macro_rules! sprint {
    ($expr:tt) => {
       print!("{}", $expr);
       flush_stdout!();
    };
    ($($arg:tt)*) => {
        print!($($arg)*);
        flush_stdout!();
    };
}

/// Simple println with one parameter
#[macro_export]
macro_rules! sprintln {
    ($expr:tt) => {
        println!("{}", $expr)
    };
    ($expr:expr) => {
        println!("{}", $expr)
    };
}

#[derive(thiserror::Error, Debug)]
pub enum CalcError {
    #[error("Chyba vyhodnocení matematického výrazu: {0}")]
    EvaluateErr(String),

    #[error("Chyba volání funkce: {0}")]
    FuncCallErr(String),

    #[error("Syntaktická chyba matematického výrazu: {0}")]
    ParseErr(String),

    #[error("Nepodařilo se převést text na BigInt.")]
    ParseBigIntErr(#[source] <BigInt as FromStr>::Err),
}
