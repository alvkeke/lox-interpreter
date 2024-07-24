
#[derive(Debug)]
pub enum Number {
    Integer(i64),
    Decimal(f64),
}
use std::ops;

use Number::{*};

impl Number {
    pub fn from(str: &str) -> Result<Number, String> {
        if str.contains('.') {
            match str.parse::<f64>() {
                Err(ex) => Err(ex.to_string()),
                Ok(d) => Ok(Number::Decimal(d)),
            }
        } else {
            match str.parse::<i64>() {
                Err(ex) => Err(ex.to_string()),
                Ok(d) => Ok(Number::Integer(d)),
            }
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

impl ops::Add<Number> for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Integer(ii), Integer(jj)) => {
                Integer(ii + jj)
            },
            (Decimal(ii), Decimal(jj)) => {
                Decimal(ii + jj)
            },
            (Integer(ii), Decimal(jj)) => {
                Decimal(ii as f64 + jj)
            },
            (Decimal(ii), Integer(jj)) => {
                Decimal(ii + jj as f64)
            }
        }
    }
}

impl ops::Sub<Number> for Number {
    type Output = Number;
    fn sub(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Integer(ii), Integer(jj)) => {
                Integer(ii - jj)
            },
            (Decimal(ii), Decimal(jj)) => {
                Decimal(ii - jj)
            },
            (Integer(ii), Decimal(jj)) => {
                Decimal(ii as f64 - jj)
            },
            (Decimal(ii), Integer(jj)) => {
                Decimal(ii - jj as f64)
            }
        }
    }
}

impl ops::Mul<Number> for Number {
    type Output = Number;
    fn mul(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Integer(ii), Integer(jj)) => {
                Integer(ii * jj)
            },
            (Decimal(ii), Decimal(jj)) => {
                Decimal(ii * jj)
            },
            (Integer(ii), Decimal(jj)) => {
                Decimal(ii as f64 * jj)
            },
            (Decimal(ii), Integer(jj)) => {
                Decimal(ii * jj as f64)
            }
        }
    }
}

impl ops::Div<Number> for Number {
    type Output = Number;
    fn div(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Integer(ii), Integer(jj)) => {
                Integer(ii / jj)
            },
            (Decimal(ii), Decimal(jj)) => {
                Decimal(ii / jj)
            },
            (Integer(ii), Decimal(jj)) => {
                Decimal(ii as f64 / jj)
            },
            (Decimal(ii), Integer(jj)) => {
                Decimal(ii / jj as f64)
            }
        }
    }
}
