use crate::{token::Token, types::Number};



pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Group (Box<Expr>),
    Literal (Token),
    Unary (Token, Box<Expr>),
    None
}

impl Expr {
    fn expression(tks: &Vec<Token>, start: usize) -> (Self, usize) {
        Self::equality(tks, start)
    }

    fn equality(tks: &Vec<Token>, start: usize) -> (Self, usize) {
        let (mut expr, adv) = Self::comparison(tks, start);
        if let Expr::None = expr {
            return (Expr::None, 0);
        }
        let mut addup_adv = adv;

        while let tk_op @ Some(Token::EqualEqual | Token::BangEqual) = tks.get(start + adv) {
            let (right, adv) = Self::comparison(tks, start);
            if let Expr::None = right {
                break;
            }
            expr = Expr::Binary(Box::new(expr), tk_op.unwrap().clone(), Box::new(right));
            addup_adv += adv + 1;     // operation token
        }

        (expr, addup_adv)
    }

    fn comparison(tks: &Vec<Token>, start: usize) -> (Self, usize) {
        let (mut expr, adv) = Self::term(tks, start);
        if let Expr::None = expr {
            return (Expr::None, 0);
        }
        let mut addup_adv = adv;

        while let tk_op @ Some(Token::Greater | Token::GreaterEqual 
                        | Token::Less | Token::LessEqual) = tks.get(start + adv) {
            let (right, adv) = Self::term(tks, start);
            if let Expr::None = right {
                break;
            }
            expr = Expr::Binary(Box::new(expr), tk_op.unwrap().clone(), Box::new(right));
            addup_adv += adv + 1;     // operation token
        }

        (expr, addup_adv)
    }

    fn term(tks: &Vec<Token>, start: usize) -> (Self, usize) {
        let (mut expr, adv) = Self::factor(tks, start);
        if let Expr::None = expr {
            return (Expr::None, 0);
        }
        let mut addup_adv = adv;

        while let tk_op @ Some(Token::Minus | Token::Plus) = tks.get(start + adv) {
            let (right, adv) = Self::factor(tks, start);
            if let Expr::None = right {
                break;
            }
            expr = Expr::Binary(Box::new(expr), tk_op.unwrap().clone(), Box::new(right));
            addup_adv += adv + 1;     // operation token
        }

        (expr, addup_adv)
    }

    fn factor(tks: &Vec<Token>, start: usize) -> (Self, usize) {
        let (mut expr, adv) = Self::unary(tks, start);
        if let Expr::None = expr {
            return (Expr::None, 0);
        }
        let mut addup_adv = adv;

        while let tk_op @ Some(Token::Slash | Token::Star) = tks.get(start + adv) {
            let (right, adv) = Self::unary(tks, start);
            if let Expr::None = right {
                break;
            }
            expr = Expr::Binary(Box::new(expr), tk_op.unwrap().clone(), Box::new(right));
            addup_adv += adv + 1;     // operation token
        }

        (expr, addup_adv)
    }

    fn unary(tks: &Vec<Token>, start: usize) -> (Self, usize) {
        match tks.get(start) {
            tk_op @ Some(Token::Bang | Token::Minus) => {
                let (expr, adv) = Self::unary(tks, start+1);
                match expr {
                    Expr::None => (Expr::None, 0),
                    expr => (Expr::Unary(tk_op.unwrap().clone(), Box::new(expr)), 1+adv),      // !/- + 
                }
            },
            _ => {
                Self::primary(tks, start)
            },
        }
    }

    fn primary(tks: &Vec<Token>, start: usize) -> (Self, usize) {
        match tks.get(start) {
            tk @ Some(Token::False | Token::True | Token::Nil) => (Expr::Literal(tk.unwrap().clone()), 1),
            tk @ Some(Token::String(_) | Token::Number(_)) => (Expr::Literal(tk.unwrap().clone()), 1),
            Some(Token::LeftParen) => {
                let (expr, adv) = Self::expression(tks, start+1);

                if !matches!(expr, Expr::None) && 
                    matches!(tks.get(start+adv+1), Some(Token::RightParen)) {
                    (Expr::Group(Box::new(expr)), 2 + adv)  // L/R Paren + increased index(adv)
                } else {
                    (Expr::None, 0)
                }
            },
            _ => (Expr::None, 0),
        }
    }

    fn synchronize(tks: &Vec<Token>, start: usize) -> usize {
        let mut idx: usize = 0;
        while let Some(tk) = tks.get(start + idx) {
            match tk {
                Token::Semicolon => return idx+1,
                Token::Class | Token::Fun |
                    Token::Var | Token::For | Token::If | 
                    Token::While | Token::Print | Token::Return => return idx,
                    _=>{},
            }
            idx += 1;
        }
        return idx;
    }

}



