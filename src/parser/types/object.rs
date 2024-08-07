
use std::fmt::Display;

use crate::{dbg_format, parser::syntax::statement::Stmt};

use super::number::Number;


#[derive(Debug)]
pub enum Object {
    Nil,
    Boolean(bool),
    Number(Number),
    String(String),
    Function(Vec<String>, Stmt),
}

impl Clone for Object {
    fn clone(&self) -> Self {
        match self {
            Self::Nil => Self::Nil,
            Self::Boolean(arg0) => Self::Boolean(arg0.clone()),
            Self::Number(arg0) => Self::Number(arg0.clone()),
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::Function(params, body) => Self::Function(params.clone(), body.clone()),
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
            Self::Function(params, body) => write!(f, "({:#?}) {:#?}", params, body),
        }
    }
}

impl Object {

    pub fn is_true(&self) -> Result<bool, String> {
        match self {
            Self::Boolean(b) => Ok(*b),
            Self::Nil => Ok(false),
            _ => Err(dbg_format!("not a Boolean value")),
        }
    }

    pub fn logic_and(&self, right: &Object) -> Result<Object, String> {
        Ok(Object::Boolean(self.is_true()? && right.is_true()?))
    }

    pub fn logic_or(&self, right: &Object) -> Result<Object, String> {
        Ok(Object::Boolean(self.is_true()? || right.is_true()?))
    }

}


impl Object {
    pub fn not(&self) -> Result<Self, String> {
        use Object::{Boolean, Nil};
        match self {
            Boolean(bool) => Ok(Object::Boolean(!bool)),
            Nil => Ok(Object::Boolean(false)),   // treat Nil as `false`
            _ => Err(dbg_format!("not supported operation `Not(!)' on {:#?}", self))
        }
    }

    pub fn neg(&self) -> Result<Self, String> {
        match self {
            Object::Number(num) => Ok(Object::Number(-num.clone())),
            _ => Err(dbg_format!("not supported operation `Not(!)' on {:#?}", self))
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
            _ => Err(dbg_format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn sub(&self, rhs: &Self) -> Result<Self, String> {
        use Object::*;
        match (self, rhs) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Number(arg1.sub_ref(arg2)?))
            },
            _ => Err(dbg_format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn mul(&self, rhs: &Self) -> Result<Self, String> {
        use Object::*;
        match (self, rhs) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Number(arg1.mul_ref(arg2)?))
            },
            _ => Err(dbg_format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn div(&self, rhs: &Self) -> Result<Self, String> {
        use Object::*;
        match (self, rhs) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Number(arg1.div_ref(arg2)?))
            },
            _ => Err(dbg_format!("object type not allowed {:#?} == {:#?}", self, rhs)),
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
            _ => Err(dbg_format!("object type not allowed {:#?} == {:#?}", self, other)),
        }
    }

    pub fn le(&self, other: &Self) -> Result<Self, String> {
        use Object::*;
        match (self, other) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Boolean(arg1 <= arg2))
            },
            _ => Err(dbg_format!("object type not allowed {:#?} == {:#?}", self, other)),
        }
    }

    pub fn gt(&self, other: &Self) -> Result<Self, String> {
        use Object::*;
        match (self, other) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Boolean(arg1 > arg2))
            },
            _ => Err(dbg_format!("object type not allowed {:#?} == {:#?}", self, other)),
        }
    }

    pub fn ge(&self, other: &Self) -> Result<Self, String> {
        use Object::*;
        match (self, other) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Boolean(arg1 >= arg2))
            },
            _ => Err(dbg_format!("object type not allowed {:#?} == {:#?}", self, other)),
        }
    }

}

