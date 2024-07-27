
use std::fmt::Display;

use super::number::Number;


#[derive(Debug)]
pub enum Object {
    Nil,
    Boolean(bool),
    Number(Number),
    String(String),
}

impl Clone for Object {
    fn clone(&self) -> Self {
        match self {
            Self::Nil => Self::Nil,
            Self::Boolean(arg0) => Self::Boolean(arg0.clone()),
            Self::Number(arg0) => Self::Number(arg0.clone()),
            Self::String(arg0) => Self::String(arg0.clone()),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "(Nil)"),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Number(num) => write!(f, "{}", num),
            Self::String(str) => write!(f, "{}", str),
        }
    }
}

impl Object {

    pub fn is_true(&self) -> Result<bool, String> {
        match self {
            Self::Boolean(b) => Ok(*b),
            Self::Nil => Ok(false),
            _ => Err("not a Boolean value".to_string()),
        }
    }

}


impl Object {
    pub fn not(&self) -> Result<Self, String> {
        use Object::{Boolean, Nil};
        match self {
            Boolean(bool) => Ok(Object::Boolean(!bool)),
            Nil => Ok(Object::Boolean(false)),   // treat Nil as `false`
            _ => Err(format!("not supported operation `Not(!)' on {:#?}", self))
        }
    }

    pub fn neg(&self) -> Result<Self, String> {
        match self {
            Object::Number(num) => Ok(Object::Number(-num.clone())),
            _ => Err(format!("not supported operation `Not(!)' on {:#?}", self))
        }
    }

    pub fn add(&self, rhs: &Self) -> Result<Self, String> {
        use Object::*;
        match (self, rhs) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Number(arg1.add_ref(&arg2)?))
            },
            (String(arg1), String(arg2)) => {
                Ok(Object::String(format!("{}{}", arg1, arg2)))
            },
            (Number(arg1), String(arg2)) => {
                Ok(Object::String(format!("{}{}", arg1, arg2)))
            },
            (String(arg1), Number(arg2)) => {
                Ok(Object::String(format!("{}{}", arg1, arg2)))
            },
            _ => Err(format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn sub(&self, rhs: &Self) -> Result<Self, String> {
        use Object::*;
        match (self, rhs) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Number(arg1.sub_ref(arg2)?))
            },
            _ => Err(format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn mul(&self, rhs: &Self) -> Result<Self, String> {
        use Object::*;
        match (self, rhs) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Number(arg1.mul_ref(arg2)?))
            },
            _ => Err(format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn div(&self, rhs: &Self) -> Result<Self, String> {
        use Object::*;
        match (self, rhs) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Number(arg1.div_ref(arg2)?))
            },
            _ => Err(format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn eq(&self, other: &Self) -> Result<Self, String> {
        use Object::*;
        match (self, other) {
            (Nil, Nil) => Ok(Object::Boolean(true)),
            (Boolean(arg1), Boolean(arg2)) => {
                Ok(Object::Boolean(arg1 == arg2))
            },
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Boolean(arg1 == arg2))
            },
            (String(arg1), String(arg2)) => {
                Ok(Object::Boolean(arg1 == arg2))
            },
            // false if type mismatch
            _ => Ok(Object::Boolean(false)),
        }
    }

    pub fn ne(&self, other: &Self) -> Result<Self, String> {
        use Object::Boolean;
        match self.eq(other)? {
            Boolean(b) => Ok(Self::Boolean(!b)),
            _ => panic!("should not get type beyond ObjectContent::Boolean()"),
        }
    }

    pub fn lt(&self, other: &Self) -> Result<Self, String> {
        use Object::*;
        match (self, other) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Boolean(arg1 < arg2))
            },
            _ => Err(format!("object type not allowed {:#?} == {:#?}", self, other)),
        }
    }

    pub fn le(&self, other: &Self) -> Result<Self, String> {
        use Object::*;
        match (self, other) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Boolean(arg1 <= arg2))
            },
            _ => Err(format!("object type not allowed {:#?} == {:#?}", self, other)),
        }
    }

    pub fn gt(&self, other: &Self) -> Result<Self, String> {
        use Object::*;
        match (self, other) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Boolean(arg1 > arg2))
            },
            _ => Err(format!("object type not allowed {:#?} == {:#?}", self, other)),
        }
    }

    pub fn ge(&self, other: &Self) -> Result<Self, String> {
        use Object::*;
        match (self, other) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Boolean(arg1 >= arg2))
            },
            _ => Err(format!("object type not allowed {:#?} == {:#?}", self, other)),
        }
    }

}

