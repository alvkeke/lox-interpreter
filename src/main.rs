
use std::{self, io::{self, Write}};

mod token;

fn main() {
    let stdin = io::stdin();

    loop {
        let mut input_buffer: String = String::new();
        print!("");
        stdin.read_line(&mut input_buffer).unwrap();
        let mut tokens: Vec<token::Token> = Vec::new();
        if let Err(errmsg) = token::scan_from_line(&input_buffer, &mut tokens) {
            eprintln!("failed to parse the input line: {}", errmsg);
        } else {
            dbg!("{}", tokens);
        };

        io::stdout().flush().unwrap();
        break;  // for test
    }

}
