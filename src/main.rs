mod parser;

use parser::LoxParser;


fn main() {

    let mut lox = LoxParser::new();
    lox.repl();
    
}
