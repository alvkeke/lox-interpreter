
use std::fmt::Display;

use crate::{dbg_format, syntax::statement::Stmt};

use super::{common::{Crc, Result, SharedStr, shared_str_from}, number::Number};


#[derive(Debug)]
pub enum Object {
    Nil,
    Boolean(bool),
    Number(Number),
    String(SharedStr),
    Function(Vec<SharedStr>, Stmt),
}

impl Object {
    pub fn new_string(s: String) -> Self {
        Self::String(shared_str_from(s))
    }
}

pub type ObjectRc = Crc<Object>;

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

    pub fn is_true(&self) -> Result<bool> {
        match self {
            Self::Boolean(b) => Ok(*b),
            Self::Nil => Ok(false),
            _ => Err(dbg_format!("not a Boolean value")),
        }
    }

    pub fn not(&self) -> Result<Object> {
        use Object::{Boolean, Nil};
        match self {
            Boolean(bool) => Ok(Object::Boolean(!bool)),
            Nil => Ok(Object::Boolean(false)),   // treat Nil as `false`
            _ => Err(dbg_format!("not supported operation `Not(!)' on {:#?}", self))
        }
    }

    pub fn neg(&self) -> Result<Object> {
        match self {
            Object::Number(num) => Ok(Object::Number(-num.clone())),
            _ => Err(dbg_format!("not supported operation `Not(!)' on {:#?}", self))
        }
    }

    pub fn logic_and(&self, rhs: &Self) -> Result<Object> {
        Ok(Object::Boolean(self.is_true()? && rhs.is_true()?))
    }

    pub fn logic_or(&self, rhs: &Self) -> Result<Object> {
        Ok(Object::Boolean(self.is_true()? || rhs.is_true()?))
    }

    pub fn add(&self, rhs: &Self) -> Result<Object> {
        use Object::*;
        match (self, rhs) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Number(arg1.add_ref(&arg2)?))
            },
            (String(arg1), String(arg2)) => {
                Ok(Object::new_string(format!("{}{}", arg1, arg2)))
            },
            (Number(arg1), String(arg2)) => {
                Ok(Object::new_string(format!("{}{}", arg1, arg2)))
            },
            (String(arg1), Number(arg2)) => {
                Ok(Object::new_string(format!("{}{}", arg1, arg2)))
            },
            _ => Err(dbg_format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn sub(&self, rhs: &Self) -> Result<Object> {
        use Object::*;
        match (self, rhs) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Number(arg1.sub_ref(arg2)?))
            },
            _ => Err(dbg_format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn mul(&self, rhs: &Self) -> Result<Object> {
        use Object::*;
        match (self, rhs) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Number(arg1.mul_ref(arg2)?))
            },
            _ => Err(dbg_format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn div(&self, rhs: &Self) -> Result<Object> {
        use Object::*;
        match (self, rhs) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Number(arg1.div_ref(&arg2)?))
            },
            _ => Err(dbg_format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn eq(&self, rhs: &Self) -> Result<Object> {
        use Object::*;
        match (self, rhs) {
            (Nil, Nil) => Ok(Object::Boolean(true)),
            (Boolean(arg1), Boolean(arg2)) => {
                Ok(Object::Boolean(arg1 == &arg2.clone()))
            },
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Boolean(arg1 == &arg2.clone()))
            },
            (String(arg1), String(arg2)) => {
                Ok(Object::Boolean(arg1 == &arg2.clone()))
            },
            // false if type mismatch
            _ => Ok(Object::Boolean(false)),
        }
    }

    pub fn ne(&self, rhs: &Self) -> Result<Object> {
        use Object::Boolean;
        match self.eq(rhs)? {
            Boolean(b) => Ok(Self::Boolean(!b)),
            _ => panic!("should not get type beyond ObjectContent::Boolean()"),
        }
    }

    pub fn lt(&self, rhs: &Self) -> Result<Object> {
        use Object::*;
        match (self, rhs) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Boolean(arg1 < arg2))
            },
            _ => Err(dbg_format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn le(&self, rhs: &Self) -> Result<Object> {
        use Object::*;
        match (self, rhs) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Boolean(arg1 <= arg2))
            },
            _ => Err(dbg_format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn gt(&self, rhs: &Self) -> Result<Object> {
        use Object::*;
        match (self, rhs) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Boolean(arg1 > arg2))
            },
            _ => Err(dbg_format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn ge(&self, rhs: &Self) -> Result<Object> {
        use Object::*;
        match (self, rhs) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::Boolean(arg1 >= arg2))
            },
            _ => Err(dbg_format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

}

impl Object {

    pub fn to_rc(self) -> ObjectRc {
        Crc::new(self)
    }

    pub fn not_rc(&self) -> Result<ObjectRc> {
        Ok(Object::not(self)?.to_rc())
    }

    pub fn neg_rc(&self) -> Result<ObjectRc> {
        Ok(Object::neg(self)?.to_rc())
    }

    pub fn logic_and_rc(&self, rhs: ObjectRc) -> Result<ObjectRc> {
        Ok(Object::logic_and(self, &*rhs)?.to_rc())
    }

    pub fn logic_or_rc(&self, rhs: ObjectRc) -> Result<ObjectRc> {
        Ok(Object::logic_or(self, &*rhs)?.to_rc())
    }

    pub fn add_rc(&self, rhs: ObjectRc) -> Result<ObjectRc> {
        Ok(Object::add(self, &*rhs)?.to_rc())
    }

    pub fn sub_rc(&self, rhs: ObjectRc) -> Result<ObjectRc> {
        Ok(Object::sub(self, &*rhs)?.to_rc())
    }

    pub fn mul_rc(&self, rhs: ObjectRc) -> Result<ObjectRc> {
        Ok(Object::mul(self, &*rhs)?.to_rc())
    }

    pub fn div_rc(&self, rhs: ObjectRc) -> Result<ObjectRc> {
        Ok(Object::div(self, &*rhs)?.to_rc())
    }

    pub fn eq_rc(&self, rhs: ObjectRc) -> Result<ObjectRc> {
        Ok(Object::eq(self, &*rhs)?.to_rc())
    }

    pub fn ne_rc(&self, rhs: ObjectRc) -> Result<ObjectRc> {
        Ok(Object::ne(self, &*rhs)?.to_rc())
    }

    pub fn lt_rc(&self, rhs: ObjectRc) -> Result<ObjectRc> {
        Ok(Object::lt(self, &*rhs)?.to_rc())
    }

    pub fn le_rc(&self, rhs: ObjectRc) -> Result<ObjectRc> {
        Ok(Object::le(self, &*rhs)?.to_rc())
    }

    pub fn gt_rc(&self, rhs: ObjectRc) -> Result<ObjectRc> {
        Ok(Object::gt(self, &*rhs)?.to_rc())
    }

    pub fn ge_rc(&self, rhs: ObjectRc) -> Result<ObjectRc> {
        Ok(Object::ge(self, &*rhs)?.to_rc())
    }

}
