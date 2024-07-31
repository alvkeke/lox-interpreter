use std::collections::HashMap;

use crate::{dbg_format, parser::types::object::Object};

#[derive(Debug)]
pub struct VmVarPool {
    pool: HashMap<String, Object>
}

impl VmVarPool {
    pub fn new() -> Self {
        Self {
            pool: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.pool.clear()
    }

    /**
     * add an variable to the pool, overwrite if there is an exist variable
     *
     * name: name of the variable
     * obj: obj value
     */
    pub fn var_add(&mut self, name: String, obj: Object) {
        self.pool.insert(name, obj);
    }

    /**
     * set an exist variable with name, return Err(msg) if variable not exist
     *
     * name: name of the variable
     * obj: new value
     *
     * ret: the Ok(obj) if success, Err(msg) if failed
     */
    pub fn var_set(&mut self, name: String, obj: Object) -> Result<Object, String>{
        match self.pool.contains_key(&name) {
            true => {
                self.var_add(name, obj.clone());
                Ok(obj)
            },
            false => {
                Err(dbg_format!("cannot find object named: {}", name))
            }
        }
    }

    /**
     * remove/pop an variable from the pool
     *
     * name: name of the variable
     *
     * ret: Some(obj) if exist, None for not exist
     */
    pub fn var_pop(&mut self, name: &String) -> Result<Object, String> {
        match self.pool.remove(name) {
            Some(obj) => Ok(obj),
            None => Err(dbg_format!("cannot find object named: {}", name)),
        }
    }

    /**
     * get the variable value(ref)
     *
     * name: name of the variable
     *
     * ret: Some(&obj) if success, None for failed
     */
    pub fn var_get(&self, name: &String) -> Result<&Object, String> {
        match self.pool.get(name) {
            Some(obj) => Ok(obj),
            None => Err(dbg_format!("cannot find object named: {}", name)),
        }
    }

    pub fn var_get_mut(&mut self, name: &String) -> Result<&mut Object, String> {
        match self.pool.get_mut(name) {
            Some(obj) => Ok(obj),
            None => Err(dbg_format!("cannot find object named: {}", name)),
        }
    }

    pub fn var_exist(&self, name: &String) -> bool {
        self.pool.contains_key(name)
    }

}
