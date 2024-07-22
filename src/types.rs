
#[derive(Debug)]
pub enum Number {
    Integer(i64),
    Decimal(f64),
}
use Number::{*};

enum NumberOperation {
    Add,
    Sub,
    Mul,
    Div,
}
use NumberOperation::{*};


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

    fn operate(&self, target: &Number, op: NumberOperation) -> Result<Number, &str>{
        match (self, target) {
            (Integer(ii), Integer(jj)) => {
                Ok(match op {
                    Add => Integer(ii + jj),
                    Sub => Integer(ii - jj),
                    Mul => Integer(ii * jj),
                    Div => Integer(ii / jj),
                })
            },
            (Decimal(ii), Decimal(jj)) => {
                Ok(match op {
                    Add => Decimal(ii + jj),
                    Sub => Decimal(ii - jj),
                    Mul => Decimal(ii * jj),
                    Div => Decimal(ii / jj),
                })
            },
            (Integer(ii), Decimal(jj)) => {
                let fii = *ii as f64;
                Ok(match op {
                    Add => Decimal(fii + jj),
                    Sub => Decimal(fii - jj),
                    Mul => Decimal(fii * jj),
                    Div => Decimal(fii / jj),
                })
            },
            (Decimal(ii), Integer(jj)) => {
                let fjj = *jj as f64;
                Ok(match op {
                    Add => Decimal(ii + fjj),
                    Sub => Decimal(ii - fjj),
                    Mul => Decimal(ii * fjj),
                    Div => Decimal(ii / fjj),
                })
            },
            _ => {Err("unsupported operation")},
        }
    }

    pub fn add(&self, target: &Number) -> Result<Number, &str>{
        self.operate(target, NumberOperation::Add)
    }
    pub fn sub(&self, target: &Number) -> Result<Number, &str>{
        self.operate(target, NumberOperation::Sub)
    }
    pub fn mul(&self, target: &Number) -> Result<Number, &str>{
        self.operate(target, NumberOperation::Mul)
    }
    pub fn div(&self, target: &Number) -> Result<Number, &str>{
        self.operate(target, NumberOperation::Div)
    }

}





