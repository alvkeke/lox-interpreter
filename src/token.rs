use core::panic;

#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Minus,
    Plus,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(String),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF
}


fn scan_from_snap(snap: &String) -> Result<Vec<Token>, String> {
    let mut result: Vec<Token> = Vec::new();
    if let Err(msg) = scan_from_line(snap, &mut result) {
        return Err(msg);
    };

    return Ok(result);
}


enum ParseType {
    Identifier,
    Number,
}

fn find_to_close(close_ch: char, str: &mut impl Iterator<Item = char>, out_buf: &mut String) -> Result<i32, String> {
    while let Some(ch) = str.next() {
        if ch == close_ch {
            return Ok(0);
        }
        out_buf.push(ch);
    }

    return Err(format!("end without close mark: {}", close_ch));
}

pub fn scan_from_line(line: &String, list: &mut Vec<Token>) -> Result<i32, String> {

    let mut parse_type = ParseType::Identifier;
    let mut string_buffer: String = String::new();
    let mut line_itr = line.chars().peekable();

    let buf = &mut string_buffer;

    while let Some(ch) = line_itr.next() {

        if matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
            match ch {
                '0'..='9' => {
                    if buf.is_empty() && matches!(parse_type, ParseType::Identifier) {
                        parse_type = ParseType::Number;
                    }
                    buf.push(ch);
                },
                _ => {
                    if let ParseType::Number = parse_type {
                        return Err(String::from("got unexpected char during parsing the number"));
                    }
                    buf.push(ch);
                },
            }
            continue;
        }

        if !buf.is_empty() {
            let label_new = buf.clone();
            list.push(match parse_type {
                ParseType::Number => Token::Number(label_new),
                ParseType::Identifier => {
                    // check keywords before treat it as an identifier
                    match label_new.as_str() {
                        "and" => Token::And,
                        "class" => Token::Class,
                        "else" => Token::Else,
                        "false" => Token::False,
                        "for" => Token::For,
                        "fun" => Token::Fun,
                        "if" => Token::If,
                        "nil" => Token::Nil,
                        "or" => Token::Or,
                        "print" => Token::Print,
                        "return" => Token::Return,
                        "super" => Token::Super,
                        "this" => Token::This,
                        "true" => Token::True,
                        "var" => Token::Var,
                        "while" => Token::While,
                        _ => Token::Identifier(label_new),
                    }
                },
            });
            parse_type = ParseType::Identifier;
            buf.clear();
        }

        let peeked = line_itr.peek();
        match (ch, peeked) {
            // None is not avaiable
            ('"', Some(_)) => {
                if let Err(msg) = find_to_close(ch, &mut line_itr, buf) {
                    return Err(msg);
                }
                list.push(Token::String(buf.clone()));
                buf.clear();
            },
            ('(', _) => list.push(Token::LeftParen),
            (')', _) => list.push(Token::RightParen),
            ('{', _) => list.push(Token::LeftBrace),
            ('}', _) => list.push(Token::RightBrace),

            (',', _) => list.push(Token::Comma),
            ('.', _) => list.push(Token::Dot),
            (';', _) => list.push(Token::Semicolon),

            ('!' | '=' | '<' | '>', Some('=')) => {
                match ch {
                    '!' => list.push(Token::BangEqual),
                    '=' => list.push(Token::EqualEqual),
                    '<' => list.push(Token::LessEqual),
                    '>' => list.push(Token::GreaterEqual),
                    _ => panic!("should not got wrong char here"),
                }
                line_itr.next();
            },

            ('+', _) => list.push(Token::Plus),
            ('-', _) => list.push(Token::Minus),
            ('*', _) => list.push(Token::Star),
            ('/', _) => list.push(Token::Slash),

            ('!', _) => list.push(Token::Bang),
            ('=', _) => list.push(Token::Equal),
            ('<', _) => list.push(Token::Less),
            ('>', _) => list.push(Token::Greater),

            (' ' | '\r' | '\n' | '\t' , _) => {
                // println!("allowed white space, skip: {}", ch);
            }

            (_, _) => {
                return Err(format!("encounter unepxected char: {}, next: {:?}", ch, peeked));
            },
        };
    }

    return Ok(0);
}

