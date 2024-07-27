mod parser;

use parser::LoxParser;


fn main() {

    let mut lox = LoxParser::new();
    if let Err(msg) = lox.repl() {
        eprintln!("failed to read line: {}", msg);
    }

}
