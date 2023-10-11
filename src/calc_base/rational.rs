use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;
use num_bigint::BigInt;
use crate::calc_base::{MathParseError};
use num_integer::Integer;
use num_traits::{Signed, ToPrimitive};
use regex::*;
use crate::{map_err, s};

// Racionální číslo (zlomek) je chápáno jako dvojice celých čísel. Proto je počítání s ním dokonale přesné.
#[derive(Debug, Clone, Default)]
pub struct Rational{
    pub numerator: BigInt,
    pub denominator: BigInt
}

impl Rational {
    pub fn to_real(&self) -> Option<f64> {
        Some(self.numerator.to_f64()?
            / self.denominator.to_f64()?)
    }

    pub fn from_int(i: i64) -> Rational {
        Rational {
            numerator: BigInt::from(i),
            denominator: BigInt::from(1)
        }
    }

    pub fn from_bigint(i: super::BigInt) -> Rational {
        Rational {
            numerator: i,
            denominator: BigInt::from(1)
        }
    }

    pub fn new(numerator: i64, denominator: i64) -> Rational {
        Rational {
            numerator: BigInt::from(numerator),
            denominator: BigInt::from(denominator)
        }.reduce_move()
    }

    pub fn new_bigint(numerator: super::BigInt, denominator: super::BigInt) -> Rational {
        Rational {
            numerator,
            denominator
        }.reduce_move()
    }

    pub fn inverse(&self) -> Rational {
        Rational {
            numerator: self.denominator.clone(),
            denominator: self.numerator.clone()
        }
    }

    pub fn reduce_move(mut self) -> Self{
        let gcd = self.numerator.gcd(&self.denominator);
        if gcd.abs() > BigInt::from(1) {
            self.numerator = &self.numerator / &gcd;
            self.denominator = &self.denominator / &gcd;
        }
        self
    }

    /// Když má zlomek jmenovatel 1, dá se považovat za celé číslo
    pub fn to_bigint(&self) -> Option<BigInt> {
        return if self.denominator == BigInt::from(1) {
            Some(self.numerator.clone())
        } else {
            None
        }
    }
}

/// Každé číslo napsané posloupností číslic je racionální, např. -52.464864686
/// Není dobré pracovat s takovými čísly jako s f64, protože se ztratí přesnost.
impl FromStr for Rational {
    type Err = MathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Například: -52.708 = -52708 / 1000
        let num_regex = Regex::new(r"^(?<p1>\d+)((\.)(?<p2>\d+))?$").unwrap();
        let captures = num_regex.captures(s)
            .ok_or(MathParseError::new(s!("Nepodařilo se extrahovat části čísla regulárním výrazem")))?;

        let p1_str = captures.name("p1").ok_or(MathParseError::new(s!("Racionální číslo má mít tvar #.#")))?.as_str().trim().trim_start_matches('0');
        let p2_str = captures.name("p2").ok_or(MathParseError::new(s!("Racionální číslo má mít tvar #.#")))?.as_str().trim().trim_end_matches('0');
        return if p2_str.is_empty() { // Byly to jen nuly
            let numerator = map_err!(p1_str.parse::<BigInt>(), MathParseError,"Nepodařilo se převést výraz na racionální číslo")?;
            Ok(Rational::from_bigint(numerator))
        } else {
            let p1 = map_err!(p1_str.parse::<BigInt>(), MathParseError, "Nepodařilo se převést výraz na racionální číslo")?;
            let p2 = map_err!(p2_str.parse::<BigInt>(), MathParseError, "Nepodařilo se převést výraz na racionální číslo")?;
            let mul = BigInt::from(10).pow(p2_str.chars().count() as u32);
            let numerator = p1 * &mul + p2;
            let denumerator = mul;

            Ok(Rational::new_bigint(numerator, denumerator))
        }
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / {}", self.numerator, self.denominator)
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let result = Rational {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator
        };
        result.reduce_move()
    }
}

impl Neg for Rational {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Rational {
            numerator: -self.numerator,
            denominator: self.denominator
        }
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (self + (-rhs)).reduce_move()
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl Add for Rational{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return if self.denominator == rhs.denominator {
            Rational {
                numerator: self.numerator + rhs.numerator,
                denominator: self.denominator
            }.reduce_move()
        } else {
            let lcm = self.denominator.lcm(&rhs.denominator);

            let self_coef = &lcm / &self.denominator;
            let rhs_coef = &lcm / &rhs.denominator;

            Rational {
                numerator: self.numerator * self_coef + rhs.numerator * rhs_coef,
                denominator: lcm
            }.reduce_move()
        }
    }
}