use crate::{token::Token, types::object::Object};


#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Group (Box<Expr>),
    Literal (Token),
    Unary (Token, Box<Expr>),
    None
}


impl Expr {
    fn visit(self) -> Result<Object, String> {
        use Expr::*;
        use Token::{*};
        match self {
            // simple values
            Literal(Nil) => Ok(Object::new()),
            Literal(False) => Ok(Object::newBool(false)),
            Literal(True) => Ok(Object::newBool(true)),
            Literal(String(str)) => Ok(Object::newString(str)),
            Literal(Number(num)) => Ok(Object::newNumber(num)),
            // Unary expr
            Unary(Bang, expr) => expr.evaluate()?.not(),
            Unary(Minus, expr) => expr.evaluate()?.neg(),
            // Group expr
            Group(expr) => expr.evaluate(),
            // Binary
            Binary(left, Slash, right) => left.evaluate()?.div(right.evaluate()?),
            Binary(left, Star, right) => left.evaluate()?.mul(right.evaluate()?),
            Binary(left, Minus, right) => left.evaluate()?.sub(right.evaluate()?),
            Binary(left, Plus, right) => left.evaluate()?.add(right.evaluate()?),
            Binary(left, Greater, right) => left.evaluate()?.gt(&right.evaluate()?),
            Binary(left, GreaterEqual, right) => left.evaluate()?.ge(&right.evaluate()?),
            Binary(left, Less, right) => left.evaluate()?.lt(&right.evaluate()?),
            Binary(left, LessEqual, right) => left.evaluate()?.le(&right.evaluate()?),
            Binary(left, EqualEqual, right) => left.evaluate()?.eq(&right.evaluate()?),
            Binary(left, BangEqual, right) => left.evaluate()?.ne(&right.evaluate()?),
            None => {
                panic!("should not reach Expr::None")
            },
            left => {
                Err(format!("NOT CHECKED TYPE: {:?}", left))
            },
        }
    }

    fn evaluate(&self) -> Result<Object, String> {
        todo!()
    }
}

// parsing methods
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



