use std::cmp::Ordering;
use std::io;
use std::io::Write;

use syntax::expression::Expr;
use syntax::statement::Stmt;
use syntax::token::Token;
use types::object::Object;
use vm::vm::LoxVM;

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

#[macro_export]
macro_rules! dbg_println {
    ($fmt:expr) => {{
        println!(
            "[{}:{}] {}",
            file!(),
            line!(),
            $fmt
        )
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        println!(
            "[{}:{}] {}",
            file!(),
            line!(),
            println!($fmt, $($arg)*)
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
    pub mod vm;
}


#[derive(Debug)]
pub struct LoxParser {
    prompt: String,
    vm: LoxVM,
    tokens: Vec<Token>,
}

// init related
impl LoxParser {
    pub fn new() -> Self {
        let lox = LoxParser{
            prompt: String::from(">> "),
            vm: LoxVM::new(),
            tokens: Vec::new()
        };

        lox
    }

    pub fn clear (&mut self) {
        self.vm.clear();
        self.tokens.clear();
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
            Literal(Identifier(idnt_name)) => Ok(self.vm.var_get(idnt_name)?.clone()),
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
                self.vm.var_set(idnt_name.clone(), value)
            },
            FnCall(fn_name, args) => {
                let (params, body) = match self.vm.var_get(fn_name)? {
                    Object::Function(params, body) => (params, body),
                    _ => return Err(dbg_format!("not a function: {}", fn_name)),
                };
                let n_params = params.len();
                if n_params != args.len() {
                    return Err(dbg_format!("function `{}` expect {} arguments, got {}", fn_name, n_params, args.len()));
                }

                // don't clone() before param length check
                let params = params.clone();
                let body = body.clone();

                let mut real_args: Vec<Object> = Vec::new();
                for arg in args {
                    real_args.push(self.eval(arg)?);
                }

                self.vm.stack_new(fn_name.clone());
                self.vm.var_add_all(params, real_args);
                self.exec(&body)?;
                self.vm.stack_del();
                // ret
                Ok(Object::Nil)
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
                self.vm.block_enter();
                let mut iter = stmts.iter();
                while let Some(stmt) = iter.next() {
                    self.exec(stmt)?;
                }
                self.vm.block_exit();
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
                        self.vm.var_add(idnt_name.clone(), obj);
                    },
                    _ => {
                        self.vm.var_add(idnt_name.clone(), Object::Nil);
                    },
                };
            },
            Stmt::FunDecl(fn_name, params, fn_body) => {
                self.vm.var_add(fn_name.clone(), Object::Function(params.clone(), *fn_body.clone()));
            },
            Stmt::While(cont, body) => {
                while self.eval(cont)?.is_true()? {
                    self.exec(body)?;
                }
            },
            Stmt::For(start, cont, every, body) => {
                self.vm.block_enter();
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
                self.vm.block_exit();
            },
            _ => {
                return Err(dbg_format!("Unexpected statement"));
            },
        }
        Ok(None)
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

