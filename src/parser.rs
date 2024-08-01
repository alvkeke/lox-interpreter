use std::cmp::Ordering;
use std::io;
use std::io::Write;

use syntax::expression::Expr;
use syntax::statement::Stmt;
use syntax::token::Token;
use types::object::Object;
use vm::stack::VmStack;

use crate::dbg_format;

#[macro_export]
macro_rules! dbg_format {
    ($fmt:expr) => {{
        format!(
            "[{}:{}] {}",
            file!(),
            line!(),
            $fmt
        )
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        format!(
            "[{}:{}] {}",
            file!(),
            line!(),
            format!($fmt, $($arg)*)
        )
    }};
}

mod syntax {
    pub mod token;
    pub mod expression;
    pub mod statement;
}
mod types {
    pub mod object;
    pub mod number;
}

mod vm {
    pub mod stack;
    pub mod var_pool;
}


#[derive(Debug)]
pub struct LoxParser {
    prompt: String,
    global: VmStack,
    stacks: Vec<VmStack>,
    tokens: Vec<Token>,
}

// init related
impl LoxParser {
    pub fn new() -> Self {
        let lox = LoxParser{
            prompt: String::from(">> "),
            global: VmStack::new("()".to_string()),
            stacks: Vec::new(),
            tokens: Vec::new()
        };

        lox
    }
}

// execute related
impl LoxParser {

    pub fn eval(&mut self, expr: &Expr) -> Result<Object, String> {
        use Expr::*;
        use Token::{*};
        match expr {
            // simple values
            Literal(Nil) => Ok(Object::Nil),
            Literal(False) => Ok(Object::Boolean(false)),
            Literal(True) => Ok(Object::Boolean(true)),
            Literal(String(str)) => Ok(Object::String(str.clone())),
            Literal(Number(num)) => Ok(Object::Number(num.clone())),
            Literal(Identifier(idnt_name)) => Ok(self.var_get(idnt_name)?.clone()),
            // Unary expr
            Unary(Bang, expr) => self.eval(expr)?.not(),
            Unary(Minus, expr) => self.eval(expr)?.neg(),
            // Group expr
            Group(expr) => self.eval(expr),
            // Binary
            Binary(left, Slash, right) => self.eval(left)?.div(&self.eval(right)?),
            Binary(left, Star, right) => self.eval(left)?.mul(&self.eval(right)?),
            Binary(left, Minus, right) => self.eval(left)?.sub(&self.eval(right)?),
            Binary(left, Plus, right) => self.eval(left)?.add(&self.eval(right)?),
            Binary(left, Greater, right) => self.eval(left)?.gt(&self.eval(right)?),
            Binary(left, GreaterEqual, right) => self.eval(left)?.ge(&self.eval(right)?),
            Binary(left, Less, right) => self.eval(left)?.lt(&self.eval(right)?),
            Binary(left, LessEqual, right) => self.eval(left)?.le(&self.eval(right)?),
            Binary(left, EqualEqual, right) => self.eval(left)?.eq(&self.eval(right)?),
            Binary(left, BangEqual, right) => self.eval(left)?.ne(&self.eval(right)?),
            Binary(left, And, right) => self.eval(left)?.logic_and(&self.eval(right)?),
            Binary(left, Or, right) => self.eval(left)?.logic_or(&self.eval(right)?),
            Assign(Identifier(idnt_name), expr) => {
                let value = self.eval(expr)?;
                self.var_set(idnt_name.clone(), value)
            },
            left => {
                Err(dbg_format!("NOT CHECKED TYPE: {:#?}", left))
            },
        }
    }

    pub fn exec(&mut self, stmt: &Stmt) -> Result<Option<i32>, String> {
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
                        self.var_add(idnt_name.clone(), Object::Nil);
                    },
                };
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
        Ok(None)
    }

}

// VirtualMachine related
impl LoxParser {

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
        self.stacks.remove(0);
    }

    /**
     * get current stack, will return `global` if no function stack exist
     */
    pub fn stack_current(&self) -> &VmStack {
        if self.stacks.is_empty() {
            &self.global
        } else {
            self.stacks.get(0).unwrap()
        }
    }

    pub fn stack_current_mut(&mut self) -> &mut VmStack {
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
        self.stack_current_mut().var_set(name, obj)
    }

    /**
     * remove/pop the variable, will go through current stack and global
     * `current stack` will go first before `global`
     *
     * name: name of the variable
     */
    #[allow(dead_code)]
    pub fn var_pop(&mut self, name: &String) -> Result<Object, String> {
        self.stack_current_mut().var_pop(name)
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
        self.stack_current().var_get(name)
    }

    #[allow(dead_code)]
    pub fn var_get_mut(&mut self, name: &String) -> Result<&mut Object, String> {
        self.stack_current_mut().var_get_mut(name)
    }

    pub fn block_enter(&mut self) {
        self.stack_current_mut().scope_enter()
    }

    pub fn block_exit(&mut self) {
        self.stack_current_mut().scope_exit()
    }

}


// Parser related
impl LoxParser {

    #[allow(dead_code)]
    pub fn parse_token_clear(&mut self, code: &String) -> Result<(), String> {
        self.tokens.clear();
        syntax::token::scan_from_string(code, &mut self.tokens)
    }

    #[allow(dead_code)]
    pub fn parse_token_append(&mut self, code: &String) -> Result<(), String> {
        syntax::token::scan_from_string(code, &mut self.tokens)
    }

    pub fn parse_stmt(&mut self) -> Result<Stmt, String> {
        let (stmt, used) = Stmt::stmt(&self.tokens, 0)?;
        self.tokens.drain(0..used);
        Ok(stmt)
    }

    pub fn exec_stmt(&mut self, stmt: Stmt) -> Result<(), String> {
        self.exec(&stmt)?;
        Ok(())
    }

    pub fn exec_stmt_all_available(&mut self) -> Result<(), String> {
        while !self.tokens.is_empty() {
            let stmt = self.parse_stmt()?;
            self.exec_stmt(stmt)?;
        }
        Ok(())
    }

}

// REPL
impl LoxParser {

    fn is_break_cmd(cmd: &String) -> bool {
        if let Ordering::Equal = cmd.trim().cmp(".q") {
            true
        } else {
            false
        }
    }

    pub fn exec_line(&mut self, line: &String) -> Result<(), String>{
        self.parse_token_clear(line)?;
        self.exec_stmt_all_available()
    }

    #[allow(dead_code)]
    pub fn repl(&mut self) -> Result<(), String> {
        let stdin = io::stdin();

        let mut line = String::new();
        loop {
            self.prompt_disp();
            line.clear();
            if let Err(msg) = stdin.read_line(&mut line) {
                return Err(dbg_format!("{}", msg));
            }
            if Self::is_break_cmd(&line) {
                return Ok(());
            }
            match self.exec_line(&line) {
                Err(msg) => eprintln!("{}", msg),
                _=>{},
            }
        }
    }

}

// prompt related
impl LoxParser {
    #[allow(dead_code)]
    pub fn prompt_set(&mut self, prompt: String) {
        self.prompt = prompt;
    }

    #[allow(dead_code)]
    pub fn prompt_get(&self) -> &String {
        &self.prompt
    }

    pub fn prompt_disp(&self) {
        print!("{}", self.prompt);
        io::stdout().flush().unwrap();
    }
}

