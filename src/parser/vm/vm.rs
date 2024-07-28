use crate::parser::types::object::Object;

use super::stack::VmStack;


#[derive(Debug)]
pub struct LoxVM {
    global: VmStack,
    stacks: Vec<VmStack>,
}

impl LoxVM {
    pub fn new() -> LoxVM {
        LoxVM {
            global: VmStack::new("()".to_string()),
            stacks: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.global.clear();
        self.stacks.clear();
    }

    pub fn stack_new(&mut self, name: String) {
        self.stacks.insert(0, VmStack::new(name))
    }

    pub fn stack_del(&mut self) {
        self.stacks.remove(0);
    }

    /**
     * get current stack, will return `global` if no function stack exist
     */
    pub fn stack_current(&mut self) -> &mut VmStack {
        if self.stacks.is_empty() {
            &mut self.global
        } else {
            self.stacks.get_mut(0).unwrap()
        }
    }

    /**
     * add a new variable in current context,
     * overwrite if named variable exist
     *
     * name: target variable name
     * obj: value
     */
    pub fn var_add(&mut self, name: String, obj: Object) {
        self.stack_current().var_add(name, obj)
    }

    /**
     * edit the exist variable, will go through current stack and global
     * `current stack` will go first before `global`
     *
     * name: name of the variable
     */
    pub fn var_set(&mut self, name: String, obj: Object) -> Result<Object, String> {
        self.stack_current().var_set(name, obj)
    }

    /**
     * remove/pop the variable, will go through current stack and global
     * `current stack` will go first before `global`
     *
     * name: name of the variable
     */
    #[allow(dead_code)]
    pub fn var_pop(&mut self, name: &String) -> Result<Object, String> {
        self.stack_current().var_pop(name)
    }

    /**
     * get the variable value(ref), will go through current stack and global
     * `current stack` will go first before `global`
     *
     * name: name of the variable
     *
     * ret: Some(&obj) if success, None for failed
     */
    pub fn var_get(&mut self, name: &String) -> Result<&mut Object, String> {
        self.stack_current().var_get(name)
    }

    pub fn block_enter(&mut self) {
        self.stack_current().scope_enter()
    }

    pub fn block_exit(&mut self) {
        self.stack_current().scope_exit()
    }

}
