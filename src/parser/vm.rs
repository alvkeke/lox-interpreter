use std::collections::HashMap;

use crate::parser::types::object::Object;





pub struct LoxVM {
    vars: HashMap<String, Object>,
}


impl LoxVM {
    pub fn new() -> LoxVM {
        LoxVM { vars: HashMap::new() }
    }

    /**
     * add or set object, add a new object if not exist,
     * modify the value if the object exist with `name`
     */
    pub fn obj_set(&mut self, name: String, obj: Object) {
        self.vars.insert(name, obj);
    }

    pub fn obj_set_if_exist(&mut self, name: String, obj: Object) -> Result<Object, String> {
        match self.vars.contains_key(&name) {
            true => {
                self.obj_set(name, obj.clone());
                Ok(obj)
            },
            false => {
                Err(format!("cannot find object named: {}", name))
            }
        }
    }

    #[allow(dead_code)]
    fn obj_pop(&mut self, name: String) -> Option<Object> {
        self.vars.remove(&name)
    }

    pub fn obj_get(&mut self, name: &String) -> Option<&mut Object> {
        self.vars.get_mut(name)
    }

}


