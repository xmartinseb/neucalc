use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};
use num_bigint::BigInt;
use crate::calc_base::{BigInteger, Real};
use num_integer::Integer;
use num_traits::ToPrimitive;

// Racionální číslo (zlomek) je chápáno jako dvojice celých čísel. Proto je počítání s ním dokonale přesné.
#[derive(Debug, Clone)]
pub struct Rational{
    numerator: BigInteger,
    denominator: BigInteger
}

impl Rational {
    pub fn to_real(&self) -> Option<Real> {
        Some(self.numerator.to_f64()?
            / self.denominator.to_f64()?)
    }

    pub fn from_int(i: super::Integer) -> Rational {
        Rational {
            numerator: BigInteger::from(i),
            denominator: BigInteger::from(1)
        }
    }

    pub fn from_bigint(i: super::BigInteger) -> Rational {
        Rational {
            numerator: i,
            denominator: BigInteger::from(1)
        }
    }

    pub fn new(numerator: super::Integer, denominator: super::Integer) -> Rational {
        Rational {
            numerator: BigInteger::from(numerator),
            denominator: BigInteger::from(denominator)
        }
    }

    pub fn new_bigint(numerator: super::BigInteger, denominator: super::BigInteger) -> Rational {
        Rational {
            numerator,
            denominator
        }
    }

    pub fn inverse(&self) -> Rational {
        Rational {
            numerator: self.denominator.clone(),
            denominator: self.numerator.clone()
        }
    }

    pub fn reduce_move(mut self) -> Self{
        let gcd = self.numerator.gcd(&self.denominator);
        if gcd > BigInt::from(1) {
            self.numerator = &self.numerator / &gcd;
            self.denominator = &self.denominator / &gcd;
        }
        self
    }

    /// Když má zlomek jmenovatel 1, dá se považovat za celé číslo
    pub fn to_bigint(&self) -> Option<BigInteger> {
        return if self.denominator == BigInteger::from(1) {
            Some(self.numerator.clone())
        } else {
            None
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

            let self_coef = &lcm / &rhs.denominator;
            let rhs_coef = &lcm / &self.denominator;

            Rational {
                numerator: self.numerator * self_coef + rhs.numerator * rhs_coef,
                denominator: lcm
            }.reduce_move()
        }
    }
}