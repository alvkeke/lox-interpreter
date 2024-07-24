
use std::{cmp::Ordering, ops::Add};

use super::number::Number;


#[derive(Debug)]
pub struct Object {
    name: Option<String>,
    content: ObjectContent,
}

#[derive(Debug)]
pub enum ObjectContent {
    Nil,
    Boolean(bool),
    Number(Number),
    String(String),
}

// Instance and data operate methods 
impl Object {
    pub fn new() -> Self {
        Object { name: None, content: ObjectContent::Nil }
    }

    pub fn setNil(&mut self) {
        self.content = ObjectContent::Nil;
    }
    
    pub fn newBool(boolean: bool) -> Self {
        Object { name: None, content: ObjectContent::Boolean(boolean) }
    }

    pub fn setBool(&mut self, boolean: bool) {
        self.content = ObjectContent::Boolean(boolean);
    }

    pub fn newNumber(num: Number) -> Self {
        Object { name: None, content: ObjectContent::Number(num) }
    }

    pub fn setNumber(&mut self, num: Number) {
        self.content = ObjectContent::Number(num);
    }

    pub fn newString(str: String) -> Self {
        Object { name: None, content: ObjectContent::String(str) }
    }

    pub fn setString(&mut self, str: String) {
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
    pub fn not(self) -> Result<Object, String> {
        match self.content {
            ObjectContent::Boolean(bool) => Ok(Object::newBool(!bool)),
            _ => Err(format!("not supported operation `Not(!)' on {:?}", self))
        }
    }

    pub fn neg(self) -> Result<Object, String> {
        match self.content {
            ObjectContent::Number(num) => Ok(Object::newNumber(-num)),
            _ => Err(format!("not supported operation `Not(!)' on {:?}", self))
        }
    }

    pub fn add(self, rhs: Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &rhs.content) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::newNumber(arg1.add_ref(arg2)))
            },
            (String(arg1), String(arg2)) => {
                Ok(Object::newString(format!("{}{}", arg1, arg2)))
            }
            _ => Err(format!("object type not allowed {:?} == {:?}", self, rhs)),
        }
    }

    pub fn sub(self, rhs: Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &rhs.content) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::newNumber(arg1.sub_ref(arg2)))
            },
            _ => Err(format!("object type not allowed {:?} == {:?}", self, rhs)),
        }
    }

    pub fn mul(self, rhs: Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &rhs.content) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::newNumber(arg1.mul_ref(arg2)))
            },
            _ => Err(format!("object type not allowed {:?} == {:?}", self, rhs)),
        }
    }

    pub fn div(self, rhs: Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &rhs.content) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::newNumber(arg1.div_ref(arg2)))
            },
            _ => Err(format!("object type not allowed {:?} == {:?}", self, rhs)),
        }
    }

    pub fn eq(&self, other: &Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &other.content) {
            (Nil, Nil) => Ok(Object::newBool(true)),
            (Boolean(arg1), Boolean(arg2)) => {
                Ok(Object::newBool(arg1 == arg2))
            },
            (Number(arg1), Number(arg2)) => {
                Ok(Object::newBool(arg1 == arg2))
            },
            (String(arg1), String(arg2)) => {
                Ok(Object::newBool(arg1 == arg2))
            }
            _ => Err(format!("object type mismatch {:?} == {:?}", self, other)),
        }
    }
    
    pub fn ne(&self, other: &Self) -> Result<Object, String> {
        use ObjectContent::Boolean;
        // !self.eq(other)
        match self.eq(other) {
            Ok(obj) => {
                match obj.content {
                    Boolean(b) => Ok(Object::newBool(!b)),
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
                Ok(Object::newBool(arg1 < arg2))
            },
            _ => Err(format!("object type not allowed {:?} == {:?}", self, other)),
        }
    }

    pub fn le(&self, other: &Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &other.content) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::newBool(arg1 <= arg2))
            },
            _ => Err(format!("object type not allowed {:?} == {:?}", self, other)),
        }
    }

    pub fn gt(&self, other: &Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &other.content) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::newBool(arg1 > arg2))
            },
            _ => Err(format!("object type not allowed {:?} == {:?}", self, other)),
        }
    }

    pub fn ge(&self, other: &Self) -> Result<Object, String> {
        use ObjectContent::*;
        match (&self.content, &other.content) {
            (Number(arg1), Number(arg2)) => {
                Ok(Object::newBool(arg1 >= arg2))
            },
            _ => Err(format!("object type not allowed {:?} == {:?}", self, other)),
        }
    }

}

