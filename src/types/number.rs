
#[derive(Debug)]
pub enum Number {
    Integer(i64),
    Decimal(f64),
}

use std::{cmp::Ordering, ops::{self, Neg}};

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

impl Number {
    pub fn add_ref(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Integer(ii), Integer(jj)) => {
                Integer(*ii + *jj)
            },
            (Decimal(ii), Decimal(jj)) => {
                Decimal(*ii + *jj)
            },
            (Integer(ii), Decimal(jj)) => {
                Decimal(*ii as f64 + *jj)
            },
            (Decimal(ii), Integer(jj)) => {
                Decimal(*ii + *jj as f64)
            }
        }
    }

    pub fn sub_ref(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Integer(ii), Integer(jj)) => {
                Integer(*ii - *jj)
            },
            (Decimal(ii), Decimal(jj)) => {
                Decimal(*ii - *jj)
            },
            (Integer(ii), Decimal(jj)) => {
                Decimal(*ii as f64 - *jj)
            },
            (Decimal(ii), Integer(jj)) => {
                Decimal(*ii - *jj as f64)
            }
        }
    }
    pub fn mul_ref(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Integer(ii), Integer(jj)) => {
                Integer(*ii * *jj)
            },
            (Decimal(ii), Decimal(jj)) => {
                Decimal(*ii * *jj)
            },
            (Integer(ii), Decimal(jj)) => {
                Decimal(*ii as f64 * *jj)
            },
            (Decimal(ii), Integer(jj)) => {
                Decimal(*ii * *jj as f64)
            }
        }
    }

    pub fn div_ref(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Integer(ii), Integer(jj)) => {
                Integer(*ii / *jj)
            },
            (Decimal(ii), Decimal(jj)) => {
                Decimal(*ii / *jj)
            },
            (Integer(ii), Decimal(jj)) => {
                Decimal(*ii as f64 / *jj)
            },
            (Decimal(ii), Integer(jj)) => {
                Decimal(*ii / *jj as f64)
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

