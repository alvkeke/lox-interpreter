
#[derive(Debug)]
pub enum Number {
    Integer(i64),
    Decimal(f64),
}

use std::{cmp::Ordering, fmt::Display, ops::{self, Neg}};

use Number::{*};

use crate::dbg_format;
use crate::parser::types::common::Result;

impl Number {
    pub fn from(str: &str) -> Result<Number> {
        if str.contains('.') {
            match str.parse::<f64>() {
                Err(ex) => Err(dbg_format!("{}", ex)),
                Ok(d) => Ok(Number::Decimal(d)),
            }
        } else {
            match str.parse::<i64>() {
                Err(ex) => Err(dbg_format!("{}", ex)),
                Ok(d) => Ok(Number::Integer(d)),
            }
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer(ii) => write!(f, "{}", ii),
            Decimal(ii) => write!(f, "{}", ii),
        }
    }
}

impl Clone for Number {
    fn clone(&self) -> Self {
        match self {
            Self::Integer(arg0) => Self::Integer(arg0.clone()),
            Self::Decimal(arg0) => Self::Decimal(arg0.clone()),
        }
    }
}

impl Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        match self {
            Integer(ii) => Integer(-ii),
            Decimal(ff) => Decimal(-ff),
        }
    }
}

impl ops::Add<Number> for Number {
    type Output = Result<Number>;
    fn add(self, rhs: Number) -> Self::Output {
        self.add_ref(&rhs)
    }
}

impl ops::Sub<Number> for Number {
    type Output = Result<Number>;
    fn sub(self, rhs: Number) -> Self::Output {
        self.sub_ref(&rhs)
    }
}

impl ops::Mul<Number> for Number {
    type Output = Result<Number>;
    fn mul(self, rhs: Number) -> Self::Output {
        self.mul_ref(&rhs)
    }
}

impl ops::Div<Number> for Number {
    type Output = Result<Number>;
    fn div(self, rhs: Number) -> Self::Output {
        self.div_ref(&rhs)
    }
}

impl Number {

    pub fn is_zero(&self) -> bool {
        match self {
            Integer(0) => true,
            Decimal(0.0) => true,
            _ => false,
        }
    }

    pub fn add_ref(&self, rhs: &Self) -> Result<Number> {
        match (self, rhs) {
            (Integer(ii), Integer(jj)) => {
                Ok(Integer(*ii + *jj))
            },
            (Decimal(ii), Decimal(jj)) => {
                Ok(Decimal(*ii + *jj))
            },
            (Integer(ii), Decimal(jj)) => {
                Ok(Decimal(*ii as f64 + *jj))
            },
            (Decimal(ii), Integer(jj)) => {
                Ok(Decimal(*ii + *jj as f64))
            }
        }
    }

    pub fn sub_ref(&self, rhs: &Self) -> Result<Number> {
        match (self, rhs) {
            (Integer(ii), Integer(jj)) => {
                Ok(Integer(*ii - *jj))
            },
            (Decimal(ii), Decimal(jj)) => {
                Ok(Decimal(*ii - *jj))
            },
            (Integer(ii), Decimal(jj)) => {
                Ok(Decimal(*ii as f64 - *jj))
            },
            (Decimal(ii), Integer(jj)) => {
                Ok(Decimal(*ii - *jj as f64))
            }
        }
    }
    pub fn mul_ref(&self, rhs: &Self) -> Result<Number> {
        match (self, rhs) {
            (Integer(ii), Integer(jj)) => {
                Ok(Integer(*ii * *jj))
            },
            (Decimal(ii), Decimal(jj)) => {
                Ok(Decimal(*ii * *jj))
            },
            (Integer(ii), Decimal(jj)) => {
                Ok(Decimal(*ii as f64 * *jj))
            },
            (Decimal(ii), Integer(jj)) => {
                Ok(Decimal(*ii * *jj as f64))
            }
        }
    }

    pub fn div_ref(&self, rhs: &Self) -> Result<Number> {
        if rhs.is_zero() {
            return Err(dbg_format!("cannot divide by Zero: {} / {}", self, rhs));
        }
        match (self, rhs) {
            (Integer(ii), Integer(jj)) => {
                Ok(Integer(*ii / *jj))
            },
            (Decimal(ii), Decimal(jj)) => {
                Ok(Decimal(*ii / *jj))
            },
            (Integer(ii), Decimal(jj)) => {
                Ok(Decimal(*ii as f64 / *jj))
            },
            (Decimal(ii), Integer(jj)) => {
                Ok(Decimal(*ii / *jj as f64))
            }
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Integer(ii), Integer(jj)) => {
                ii.partial_cmp(jj)
            },
            (Decimal(ii), Decimal(jj)) => {
                ii.partial_cmp(jj)
            },
            (Integer(ii), Decimal(jj)) => {
                (*ii as f64).partial_cmp(jj)
            },
            (Decimal(ii), Integer(jj)) => {
                ii.partial_cmp(&(*jj as f64))
            }
        }
    }

    fn lt(&self, other: &Self) -> bool {
        std::matches!(self.partial_cmp(other), Some(Ordering::Less))
    }

    fn le(&self, other: &Self) -> bool {
        std::matches!(self.partial_cmp(other), Some(Ordering::Less | Ordering::Equal))
    }

    fn gt(&self, other: &Self) -> bool {
        std::matches!(self.partial_cmp(other), Some(Ordering::Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        std::matches!(self.partial_cmp(other), Some(Ordering::Greater | Ordering::Equal))
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Integer(ii), Integer(jj)) => {
                ii.eq(jj)
            },
            (Decimal(ii), Decimal(jj)) => {
                ii.eq(jj)
            },
            (Integer(ii), Decimal(jj)) => {
                (*ii as f64) == *jj
            },
            (Decimal(ii), Integer(jj)) => {
                *ii == (*jj as f64)
            }
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

