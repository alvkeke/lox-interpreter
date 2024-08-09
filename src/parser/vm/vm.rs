use crate::parser::types::object::Object;

use super::stack::VmStack;


#[derive(Debug)]
pub struct LoxVM {
    global: VmStack,
    stacks: Vec<VmStack>,
}

impl LoxVM {
    pub fn new () -> Self {
        Self {
            global: VmStack::new("(global)".to_string()),
            stacks: Vec::new(),
        }
    }
}


// VirtualMachine related
impl LoxVM {

    pub fn clear(&mut self) {
        self.global.clear();
        self.stacks.clear();
    }

    pub fn stack_new(&mut self, name: String) {
        self.stacks.insert(0, VmStack::new(name))
    }

    #[allow(dead_code)]
    pub fn stack_new_with_args(&mut self, stack_name: String, params: Vec<String>, args: Vec<Object>) {
        self.stack_new(stack_name);
        self.var_add_all(params, args);
    }

    pub fn stack_del(&mut self) {
        // self.env.borrow_mut().stacks.remove(0);
        self.stacks.remove(0);
    }

    /**
     * get current stack, will return `global` if no function stack exist
     */
    #[allow(dead_code)]
    pub fn stack_current(&self) -> &VmStack {
        if self.stacks.is_empty() {
            &self.global
        } else {
            &self.stacks.get(0).unwrap()
        }
    }

    pub fn stack_current_mut(&mut self) -> &mut VmStack {
        if self.stacks.is_empty() {
            &mut self.global
        } else {
            self.stacks.get_mut(0).unwrap()
        }
    }

    pub fn stack_for_var(&self, name: &String) -> &VmStack {
        let mut iter = self.stacks.iter();
        while let Some(stack) = iter.next() {
            if stack.var_exist(name) {
                return stack;
            }
        }
        &self.global
    }

    pub fn stack_for_var_mut(&mut self, name: &String) -> &mut VmStack {
        let mut iter = self.stacks.iter_mut();
        while let Some(stack) = iter.next() {
            if stack.var_exist(name) {
                return stack;
            }
        }
        &mut self.global
    }

    /**
     * add a new variable in current context,
     * overwrite if named variable exist
     *
     * name: target variable name
     * obj: value
     */
    pub fn var_add(&mut self, name: String, obj: Object) {
        self.stack_current_mut().var_add(name, obj)
    }

    #[allow(dead_code)]
    pub fn var_add_all(&mut self, mut params: Vec<String>, mut args: Vec<Object>) {
        while !params.is_empty() && !args.is_empty() {
            let name = params.remove(0);
            let obj = args.remove(0);
            self.var_add(name, obj);
        }
    }

    /**
     * edit the exist variable, will go through current stack and global
     * `current stack` will go first before `global`
     *
     * name: name of the variable
     */
    pub fn var_set(&mut self, name: String, obj: Object) -> Result<Object, String> {
        self.stack_for_var_mut(&name).var_set(name, obj)
    }

    /**
     * remove/pop the variable, will go through current stack and global
     * `current stack` will go first before `global`
     *
     * name: name of the variable
     */
    #[allow(dead_code)]
    pub fn var_pop(&mut self, name: &String) -> Result<Object, String> {
        self.stack_for_var_mut(name).var_pop(name)
    }

    /**
     * get the variable value(ref), will go through current stack and global
     * `current stack` will go first before `global`
     *
     * name: name of the variable
     *
     * ret: Some(&obj) if success, None for failed
     */
    pub fn var_get(&self, name: &String) -> Result<&Object, String> {
        self.stack_for_var(name).var_get(name)
    }

    #[allow(dead_code)]
    pub fn var_get_mut(&mut self, name: &String) -> Result<&mut Object, String> {
        self.stack_for_var_mut(name).var_get_mut(name)
    }

    pub fn block_enter(&mut self) {
        self.stack_current_mut().scope_enter()
    }

    pub fn block_exit(&mut self) {
        self.stack_current_mut().scope_exit()
    }

}

