use std::collections::HashMap;

use crate::types::object::{Object, ObjectContent};





pub struct LoxVM {
    vars: HashMap<String, Object>,
}


impl LoxVM {
    pub fn new() -> LoxVM {
        LoxVM { vars: HashMap::new() }
    }

    pub fn var_add(&mut self, obj: Object) {
        if let Some(name) = obj.get_name() {
            self.vars.insert(name.clone(), obj);
        }
        println!("dbg: mapsize: {}", self.vars.len());
    }

    pub fn var_set(&mut self, name: String, content: ObjectContent) {
        let mut obj = Object::content_new(content);
        obj.set_name(name.clone());
        self.vars.insert(name, obj);
        println!("dbg: mapsize: {}", self.vars.len());
    }

    fn var_del(&mut self, name: String) {
        self.vars.remove(&name);
        println!("dbg: mapsize: {}", self.vars.len());
    }

    pub fn var_get(&mut self, name: &String) -> Option<&mut Object> {
        println!("dbg: mapsize: {}", self.vars.len());
        self.vars.get_mut(name)
    }

    pub fn var_exist(&mut self, name: &String) -> bool {
        println!("dbg: mapsize: {}", self.vars.len());
        self.vars.contains_key(name)
    }

}


