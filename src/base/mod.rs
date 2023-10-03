use std::error::Error;

/// Makro převede text typu &str na strukturu String
#[macro_export]
macro_rules! s {
    ($str:expr) => {String::from($str)};
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
    }
}

/// Základní rozhraní chyb definovaných v aplikaci
pub trait IAppError: Error {
    fn get_msg(&self) -> &str;
}

/// Makro vytvoří strukturu s daným názvem odvozenou od IAppError.
#[macro_export]
macro_rules! define_error_type {
    ($err_type_name:ident) => {
        #[derive(Debug)]
        pub struct $err_type_name {
            msg: String,
            inner: Option<Box<dyn Error>>,
        }

        impl Display for $err_type_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.msg)
            }
        }

        impl Error for $err_type_name {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                match &self.inner {
                    Some(inn) => Some(inn.as_ref()),
                    None => None,
                }
            }
        }

        impl IAppError for $err_type_name {
            fn get_msg(&self) -> &str {
                &self.msg
            }

            // fn get_inner(&self) -> &Option<Box<dyn Error>> {
            //     &self.inner
            // }
        }

        impl $err_type_name {
            pub fn new(msg: String) -> Self {
                Self { msg, inner: None }
            }

            pub fn new_with_inner(msg: String, inner: Box<dyn Error>) -> Self {
                Self {
                    msg,
                    inner: Some(inner),
                }
            }
        }
    };
}