
extern crate lox_lib;

use lox_lib::LoxParser;

fn main() {
    let mut lox = LoxParser::new();
    lox.repl();
}

