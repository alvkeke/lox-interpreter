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


enum ParseState {
    Normal,
    String,
    Number,
}

pub fn scan_from_line(line: &String, list: &mut Vec<Token>) -> Result<i32, String> {

    let mut parse_state = ParseState::Normal;
    let mut string_buffer: String = String::new();
    let mut ch_peekable = line.chars().peekable();

    let buf = &mut string_buffer;

    while let Some(ch) = ch_peekable.next() {

        if let ParseState::String = parse_state {
            if ch == '"' {
                list.push(Token::String(buf.clone()));
                buf.clear();
                parse_state = ParseState::Normal;
            } else {
                buf.push(ch);
            }
            continue;
        }

        // parse_state != ParseState::String if reach here
        if matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
            match ch {
                '0'..='9' => {
                    if buf.is_empty() && matches!(parse_state, ParseState::Normal) {
                        // label_type = LabelType::Number;
                        parse_state = ParseState::Number;
                    }
                    buf.push(ch);
                },
                _ => {
                    if let ParseState::Number = parse_state {
                        return Err(String::from("got unexpected char during parsing the number"));
                    }
                    buf.push(ch);
                },
            }
            continue;
        }

        if !buf.is_empty() {
            let label_new = buf.clone();
            list.push(match parse_state {
                ParseState::Number => Token::Number(label_new),
                ParseState::Normal => Token::Identifier(label_new),
                ParseState::String => panic!("should not reach here, consider about the memory corruption"),
            });
            parse_state = ParseState::Normal;
            buf.clear();
        }

        let peeked = ch_peekable.peek();
        match (ch, peeked) {
            ('"', _) => parse_state = ParseState::String,
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
                ch_peekable.next();
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

