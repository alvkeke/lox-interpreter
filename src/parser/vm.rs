use std::collections::HashMap;

use crate::parser::types::object::Object;

pub struct LoxVM {
    global_objs: HashMap<String, Object>,
    stacks: Vec<HashMap<String, Object>>,
}


impl LoxVM {
    pub fn new() -> LoxVM {
        LoxVM {
            global_objs: HashMap::new(),
            stacks: Vec::new(),
        }
    }

    pub fn env_clear(&mut self) {
        self.global_objs.clear();
        self.stacks.clear();
    }
}


impl LoxVM {

    pub fn auto_obj_set(&mut self, name: String, obj: Object) {
        match self.stack_deep() {
            0 => self.global_obj_set(name, obj),
            _ => self.stack_obj_set(name, obj),
        }
    }

    pub fn auto_obj_set_if_exist(&mut self, name: String, obj: Object) -> Result<Object, String> {
        match self.stack_deep() {
            0 => self.global_obj_set_if_exist(name, obj),
            _ => {
                if self.stack_obj_exist(&name) {
                    self.stack_obj_set_if_exist(name, obj)
                } else {
                    self.global_obj_set_if_exist(name, obj)
                }
            },
        }
    }

    #[allow(dead_code)]
    fn auto_obj_pop(&mut self, name: String) -> Option<Object> {
        match self.stack_deep() {
            0 => self.global_obj_pop(name),
            _ => {
                if self.stack_obj_exist(&name) {
                    self.stack_obj_pop(name)
                } else {
                    self.global_obj_pop(name)
                }
            },
        }
    }

    pub fn auto_obj_get(&mut self, name: &String) -> Option<&mut Object> {
        match self.stack_deep() {
            0 => self.global_obj_get(name),
            _ => {
                if self.stack_obj_exist(name) {
                    self.stack_obj_get(name)
                } else {
                    self.global_obj_get(name)
                }
            },
        }
    }

    pub fn stack_new(&mut self) {
        self.stacks.push(HashMap::new())
    }

    pub fn stack_exit(&mut self) {
        self.stacks.pop();
    }

    pub fn stack_deep(&self) -> usize {
        self.stacks.len()
    }

    pub fn stack_current(&mut self) -> &mut HashMap<String, Object> {
        let len = self.stacks.iter().len()-1;
        self.stacks.get_mut(len).unwrap()
    }

    pub fn stack_obj_set(&mut self, name: String, obj: Object) {
        Self::map_obj_set(self.stack_current(), name, obj)
    }

    pub fn stack_obj_set_if_exist(&mut self, name: String, obj: Object) -> Result<Object, String> {
        Self::map_obj_set_if_exist(self.stack_current(), name, obj)
    }

    #[allow(dead_code)]
    fn stack_obj_pop(&mut self, name: String) -> Option<Object> {
        Self::map_obj_pop(self.stack_current(), name)
    }

    pub fn stack_obj_get(&mut self, name: &String) -> Option<&mut Object> {
        Self::map_obj_get(self.stack_current(), name)
    }

    fn stack_obj_exist(&mut self, name: &String) -> bool {
        self.stack_current().contains_key(name)
    }

    /**
     * add or set object, add a new object if not exist,
     * modify the value if the object exist with `name`
     */
    pub fn global_obj_set(&mut self, name: String, obj: Object) {
        Self::map_obj_set(&mut self.global_objs, name, obj)
    }

    pub fn global_obj_set_if_exist(&mut self, name: String, obj: Object) -> Result<Object, String> {
        Self::map_obj_set_if_exist(&mut self.global_objs, name, obj)
    }

    #[allow(dead_code)]
    fn global_obj_pop(&mut self, name: String) -> Option<Object> {
        Self::map_obj_pop(&mut self.global_objs, name)
    }

    pub fn global_obj_get(&mut self, name: &String) -> Option<&mut Object> {
        Self::map_obj_get(&mut self.global_objs, name)
    }

    fn map_obj_set(map: &mut HashMap<String, Object>, name: String, obj: Object) {
        map.insert(name, obj);
    }

    fn map_obj_set_if_exist(map: &mut HashMap<String, Object>, name: String, obj: Object) -> Result<Object, String>{
        match map.contains_key(&name) {
            true => {
                Self::map_obj_set(map, name, obj.clone());
                Ok(obj)
            },
            false => {
                Err(format!("cannot find object named: {}", name))
            }
        }
    }

    #[allow(dead_code)]
    fn map_obj_pop(map: &mut HashMap<String, Object>, name: String) -> Option<Object> {
        map.remove(&name)
    }

    fn map_obj_get<'a>(map: &'a mut HashMap<String, Object>, name: &String) -> Option<&'a mut Object> {
        map.get_mut(name)
    }

}


