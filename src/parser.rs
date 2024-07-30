use std::cmp::Ordering;
use std::io;
use std::io::Write;

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


// VirtualMachine related
impl LoxParser {

    pub fn clear(&mut self) {
        self.global.clear();
        self.stacks.clear();
    }

    pub fn stack_new(&mut self, name: String) {
        self.stacks.insert(0, VmStack::new(name))
    }

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
        stmt.exec(self)?;
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

