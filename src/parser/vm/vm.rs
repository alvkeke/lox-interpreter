
use super::stack::VmStack;
use crate::{
    dbg_format,
    parser::{
        syntax::{
            expression::Expr,
            statement::Stmt,
            token::Token
        },
        types::{
            object::{
                Object,
                ObjectRc,
            },
            common::Result
        },
    },
};

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


// execute related
impl LoxVM {

    pub fn eval(&mut self, expr: &Expr) -> Result<ObjectRc> {
        use Expr::*;
        use Token::{*};
        match expr {
            // simple values
            Literal(Nil) => Ok(Object::Nil.to_rc()),
            Literal(False) => Ok(Object::Boolean(false).to_rc()),
            Literal(True) => Ok(Object::Boolean(true).to_rc()),
            Literal(String(str)) => Ok(Object::String(str.clone()).to_rc()),
            Literal(Number(num)) => Ok(Object::Number(num.clone()).to_rc()),
            Literal(Identifier(idnt_name)) => Ok(self.var_get(idnt_name)?),
            // Unary expr
            Unary(Bang, expr) => self.eval(expr)?.not_rc(),
            Unary(Minus, expr) => self.eval(expr)?.neg_rc(),
            // Group expr
            Group(expr) => self.eval(expr),
            // Binary
            Binary(left, Slash, right) => self.eval(left)?.div_rc(self.eval(right)?),
            Binary(left, Star, right) => self.eval(left)?.mul_rc(self.eval(right)?),
            Binary(left, Minus, right) => self.eval(left)?.sub_rc(self.eval(right)?),
            Binary(left, Plus, right) => self.eval(left)?.add_rc(self.eval(right)?),
            Binary(left, Greater, right) => self.eval(left)?.gt_rc(self.eval(right)?),
            Binary(left, GreaterEqual, right) => self.eval(left)?.ge_rc(self.eval(right)?),
            Binary(left, Less, right) => self.eval(left)?.lt_rc(self.eval(right)?),
            Binary(left, LessEqual, right) => self.eval(left)?.le_rc(self.eval(right)?),
            Binary(left, EqualEqual, right) => self.eval(left)?.eq_rc(self.eval(right)?),
            Binary(left, BangEqual, right) => self.eval(left)?.ne_rc(self.eval(right)?),
            Binary(left, And, right) => self.eval(left)?.logic_and_rc(self.eval(right)?),
            Binary(left, Or, right) => self.eval(left)?.logic_or_rc(self.eval(right)?),
            Assign(Identifier(idnt_name), expr) => {
                let value = self.eval(expr)?;
                self.var_set(idnt_name.clone(), value)
            },
            FnCall(fn_name, args) => {

                let fn_obj = self.var_get(fn_name)?;
                let (params, body) = match fn_obj.as_ref() {
                    Object::Function(params, body) => (params, body),
                    _ => return Err(dbg_format!("not a function: {}", fn_name)),
                };

                let n_params = params.len();
                if n_params != args.len() {
                    return Err(dbg_format!("function `{}` expect {} arguments, got {}", fn_name, n_params, args.len()));
                }

                self.stack_new(fn_name.clone());
                for i in 0..n_params {
                    let arg = self.eval(args.get(i).unwrap())?;
                    let arg_name = params.get(i).unwrap().clone();
                    self.var_add(arg_name, arg);
                }

                let ret = self.exec(&body)?;
                self.stack_del();
                Ok(ret)
            },
            left => {
                Err(dbg_format!("NOT CHECKED TYPE: {:#?}", left))
            },
        }
    }

    pub fn exec(&mut self, stmt: &Stmt) -> Result<ObjectRc> {
        match stmt {
            Stmt::Expr(expr) => {
                self.eval(expr)?;
            },
            Stmt::Print(expr) => {
                println!("{}", self.eval(expr)?);
            },
            Stmt::Block(stmts) => {
                self.block_enter();
                let mut iter = stmts.iter();
                while let Some(stmt) = iter.next() {
                    self.exec(stmt)?;
                }
                self.block_exit();
            }
            Stmt::If(cont, stmt_true, opt_false) => {
                if self.eval(cont)?.is_true()? {
                    self.exec(stmt_true)?;
                } else if let Some(stmt_false) = opt_false {
                    self.exec(stmt_false)?;
                }
            },
            Stmt::Decl(Token::Identifier(idnt_name), expr) => {
                match expr {
                    Some(expr) => {
                        let obj = self.eval(expr)?;
                        self.var_add(idnt_name.clone(), obj);
                    },
                    _ => {
                        self.var_add(idnt_name.clone(), Object::Nil.to_rc());
                    },
                };
            },
            Stmt::FunDecl(fn_name, params, fn_body) => {
                self.var_add(fn_name.clone(), Object::Function(params.clone(), *fn_body.clone()).to_rc());
            },
            Stmt::While(cont, body) => {
                while self.eval(cont)?.is_true()? {
                    self.exec(body)?;
                }
            },
            Stmt::For(start, cont, every, body) => {
                self.block_enter();
                if let Some(start) = start {
                    self.exec(start)?;
                }
                loop {
                    if let Some(cont) = cont {
                        if !self.eval(cont)?.is_true()? {
                            break;
                        }
                    }
                    self.exec(body)?;

                    if let Some(every) = every {
                        self.eval(every)?;
                    }
                }
                self.block_exit();
            },
            _ => {
                return Err(dbg_format!("Unexpected statement"));
            },
        }
        Ok(Object::Nil.to_rc())
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
    pub fn stack_new_with_args(&mut self, stack_name: String, params: Vec<String>, args: Vec<ObjectRc>) {
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
    pub fn var_add(&mut self, name: String, obj: ObjectRc) {
        self.stack_current_mut().var_add(name, obj)
    }

    #[allow(dead_code)]
    pub fn var_add_all(&mut self, mut params: Vec<String>, mut args: Vec<ObjectRc>) {
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
    pub fn var_set(&mut self, name: String, obj: ObjectRc) -> Result<ObjectRc> {
        self.stack_for_var_mut(&name).var_set(name, obj)
    }

    /**
     * remove/pop the variable, will go through current stack and global
     * `current stack` will go first before `global`
     *
     * name: name of the variable
     */
    #[allow(dead_code)]
    pub fn var_pop(&mut self, name: &String) -> Result<ObjectRc> {
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
    pub fn var_get(&self, name: &String) -> Result<ObjectRc> {
        self.stack_for_var(name).var_get(name)
    }

    pub fn block_enter(&mut self) {
        self.stack_current_mut().scope_enter()
    }

    pub fn block_exit(&mut self) {
        self.stack_current_mut().scope_exit()
    }

}

