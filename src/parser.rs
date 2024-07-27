use std::cmp::Ordering;
use std::io;
use std::io::Write;

use syntax::statement::Stmt;
use syntax::token::Token;
use vm::LoxVM;


mod syntax {
    pub mod token;
    pub mod expression;
    pub mod statement;
}
mod types {
    pub mod object;
    pub mod number;
}

mod vm;


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
                return Err(msg.to_string());
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

