
use std::fmt::Display;

use super::number::Number;


#[derive(Debug)]
pub struct Object {
    name: Option<String>,
    content: ObjectContent,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(name) => write!(f, "{}={}", name, self.content),
            None => write!(f, "Object({})", self.content),
        }
    }
}

#[derive(Debug)]
pub enum ObjectContent {
    Nil,
    Boolean(bool),
    Number(Number),
    String(String),
}

impl Display for ObjectContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "(Nil)"),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Number(num) => write!(f, "{}", num),
            Self::String(str) => write!(f, "{}", str),
        }
    }
}

// Instance and data operate methods 
impl Object {
    pub fn new() -> Self {
        Object { name: None, content: ObjectContent::Nil }
    }

    pub fn nil_set(&mut self) {
        self.content = ObjectContent::Nil;
    }
    
    pub fn bool_new(boolean: bool) -> Self {
        Object { name: None, content: ObjectContent::Boolean(boolean) }
    }

    pub fn bool_set(&mut self, boolean: bool) {
        self.content = ObjectContent::Boolean(boolean);
    }

    pub fn number_new(num: Number) -> Self {
        Object { name: None, content: ObjectContent::Number(num) }
    }

    pub fn number_set(&mut self, num: Number) {
        self.content = ObjectContent::Number(num);
    }

    pub fn string_new(str: String) -> Self {
        Object { name: None, content: ObjectContent::String(str) }
    }

    pub fn string_set(&mut self, str: String) {
        self.content = ObjectContent::String(str);
    }

    // pub fn getName(&self) -> Option<&String> {
    //     match self.name {
    //         None => None,
    //         Some(name) => Some(&name),
    //     }
    // }

    // pub fn setName(&mut self, name: String) {
    //     self.name = Some(name);
    // }

}

// 
impl Object {
    pub fn not(&self) -> Result<Object, String> {
        use ObjectContent::{Boolean, Nil};
        match self.content {
            Boolean(bool) => Ok(Object::bool_new(!bool)),
            Nil => Ok(Object::bool_new(true)),   // treat Nil as `false`
            _ => Err(format!("not supported operation `Not(!)' on {:#?}", self))
        }
    }

    pub fn neg(&self) -> Result<Object, String> {
        match &self.content {
            ObjectContent::Number(num) => Ok(Object::number_new(-num.clone())),
            _ => Err(format!("not supported operation `Not(!)' on {:#?}", self))
        }
    }

    pub fn add(&self, rhs: &Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &rhs.content) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::number_new(arg1.add_ref(&arg2)?))
            },
            (String(arg1), String(arg2)) => {
                Ok(Object::string_new(format!("{}{}", arg1, arg2)))
            },
            (Number(arg1), String(arg2)) => {
                Ok(Object::string_new(format!("{}{}", arg1, arg2)))
            },
            (String(arg1), Number(arg2)) => {
                Ok(Object::string_new(format!("{}{}", arg1, arg2)))
            },
            _ => Err(format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn sub(&self, rhs: &Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &rhs.content) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::number_new(arg1.sub_ref(arg2)?))
            },
            _ => Err(format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn mul(&self, rhs: &Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &rhs.content) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::number_new(arg1.mul_ref(arg2)?))
            },
            _ => Err(format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn div(&self, rhs: &Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &rhs.content) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::number_new(arg1.div_ref(arg2)?))
            },
            _ => Err(format!("object type not allowed {:#?} == {:#?}", self, rhs)),
        }
    }

    pub fn eq(&self, other: &Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &other.content) {
            (Nil, Nil) => Ok(Object::bool_new(true)),
            (Boolean(arg1), Boolean(arg2)) => {
                Ok(Object::bool_new(arg1 == arg2))
            },
            (Number(arg1), Number(arg2)) => {
                Ok(Object::bool_new(arg1 == arg2))
            },
            (String(arg1), String(arg2)) => {
                Ok(Object::bool_new(arg1 == arg2))
            },
            // false if type mismatch
            _ => Ok(Object::bool_new(false)),
        }
    }
    
    pub fn ne(&self, other: &Self) -> Result<Object, String> {
        use ObjectContent::Boolean;
        // !self.eq(other)
        match self.eq(other) {
            Ok(obj) => {
                match obj.content {
                    Boolean(b) => Ok(Object::bool_new(!b)),
                    _ => panic!("should not get type beyond ObjectContent::Boolean()"),
                }
            },
            err => err,
        }
    }

    pub fn lt(&self, other: &Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &other.content) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::bool_new(arg1 < arg2))
            },
            _ => Err(format!("object type not allowed {:#?} == {:#?}", self, other)),
        }
    }

    pub fn le(&self, other: &Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &other.content) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::bool_new(arg1 <= arg2))
            },
            _ => Err(format!("object type not allowed {:#?} == {:#?}", self, other)),
        }
    }

    pub fn gt(&self, other: &Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &other.content) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::bool_new(arg1 > arg2))
            },
            _ => Err(format!("object type not allowed {:#?} == {:#?}", self, other)),
        }
    }

    pub fn ge(&self, other: &Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &other.content) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::bool_new(arg1 >= arg2))
            },
            _ => Err(format!("object type not allowed {:#?} == {:#?}", self, other)),
        }
    }

}

