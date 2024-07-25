use crate::types::object::{Object, ObjectContent};





pub struct LoxVM {
    vars: Vec<Object>,
}


impl LoxVM {
    pub fn new() -> LoxVM {
        LoxVM { vars: Vec::new() }
    }

    pub fn var_add(&mut self, obj: Object) {
        if let Some(name) = obj.get_name() {
            self.var_del(name);
        }
        self.vars.push(obj)
    }

    fn var_del(&mut self, name: &String) {
        let idx = self.var_index(name);
        if let Some(idx) = idx {
            self.vars.remove(idx);
        }
    }

    fn var_index(&mut self, name: &String) -> Option<usize> {
        for idx in 0..self.vars.len() {
            if let Some(obj) = self.vars.get(idx) {
                if let Some(oname) = obj.get_name() {
                    if oname.eq(name) {
                        return Some(idx);
                    }
                }
            }
        }

        None
    }

    pub fn var_find(&mut self, name: &String) -> Option<&mut Object> {

        let mut iter = self.vars.iter_mut();
        while let Some(obj) = iter.next() {
            if let Some(oname) = obj.get_name() {
                if name.eq(oname) {
                    return Some(obj);
                }
            }
        }
        None
    }

    pub fn var_exist(&mut self, name: &String) -> bool {
        match self.var_find(name) {
            Some(_) => true,
            None => false,
        }
    }

    /**
     * modify variable if it exist in list, 
     * and create a new one if NOT exist.
     * 
     * return -> bool : true if new obj created, otherwise false
     */
    pub fn var_set(&mut self, name: &String, content: ObjectContent) -> bool {
        match self.var_find(name) {
            Some(obj) => {
                obj.content_set(content);
                false
            },
            None => {
                self.vars.push(Object::content_new(content));
                true
            }
        }
    }


}


