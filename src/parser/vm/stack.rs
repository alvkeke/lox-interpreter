use crate::parser::types::object::Object;

use super::var_pool::VmVarPool;


#[derive(Debug)]
pub struct VmStack {
    name: String,
    scopes: Vec<VmVarPool>,
}

impl VmStack {
    pub fn new(name: String) -> Self {
        let mut stack = VmStack {
            name: name,
            scopes: Vec::new(),
        };

        // create a default scope
        stack.scope_enter();
        stack
    }

    pub fn clear(&mut self) {
        self.scopes.clear()
    }

    pub fn scope_enter(&mut self) {
        self.scopes.insert(0, VmVarPool::new())
    }

    pub fn scope_exit(&mut self) {
        self.scopes.remove(0);
    }

    pub fn scope_current(&mut self) -> &mut VmVarPool {
        // should always get a scope
        self.scopes.get_mut(0).unwrap()
    }

    pub fn scope_of_var(&mut self, name: &String) -> Result<&mut VmVarPool, String> {
        let mut iter = self.scopes.iter_mut();
        while let Some(scope) = iter.next() {
            if scope.var_exist(name) {
                return Ok(scope);
            }
        }
        Err(format!("cannot find object named: {}", name))
    }

    /**
     * add a new variable to current scope,
     * overwrite if named variable exist
     *
     * name: target variable name
     * obj: value
     */
    pub fn var_add(&mut self, name: String, obj: Object) {
        self.scope_current().var_add(name, obj)
    }

    /**
     * edit the exist variable, will go through all exist scopes
     *
     * name: name of the variable
     */
    pub fn var_set(&mut self, name: String, obj: Object) -> Result<Object, String> {
        self.scope_of_var(&name)?.var_set(name, obj)
    }

    /**
     * remove/pop the variable, will go through all exist scopes
     *
     * name: name of the variable
     */
    #[allow(dead_code)]
    pub fn var_pop(&mut self, name: &String) -> Result<Object, String> {
        self.scope_of_var(name)?.var_pop(name)
    }

    /**
     * get the variable value(ref), will go through all exist scopes
     *
     * name: name of the variable
     *
     * ret: Some(&obj) if success, None for failed
     */
    pub fn var_get(&mut self, name: &String) -> Result<&mut Object, String> {
        self.scope_of_var(name)?.var_get(&name)
    }

}
