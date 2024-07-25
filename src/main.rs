
use std::{self, io::{self, Write}};

use syntax::statement::Stmt;

mod syntax {
    pub mod token;
    pub mod expression;
    pub mod statement;
}
mod types {
    pub mod object;
    pub mod number;
}

fn main() {
    let stdin = io::stdin();

    loop {
        let mut input_buffer: String = String::new();
        print!(">> ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input_buffer).unwrap();
        let mut tokens: Vec<syntax::token::Token> = Vec::new();
        if let Err(errmsg) = syntax::token::scan_from_line(&input_buffer, &mut tokens) {
            eprintln!("failed to parse the input line: {}", errmsg);
        } else {
            dbg!(&tokens);
        };

        // let mut stmts: Vec<Stmt> = Vec::new();
        // while let 
        while !tokens.is_empty() {
            match Stmt::stmt(&tokens, 0) {
                Ok((stmt, used)) => {
                    tokens.remove(used);
                    match stmt.exec() {
                        Err(msg) => println!("{}", msg),
                        _ => {},
                    }
                },
                Err(msg) => {
                    println!("{}", msg);
                    break;
                }
            }
        }

        // break;  // for test
    }

}
