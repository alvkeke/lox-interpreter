use crate::{syntax::token::Token, types::object::Object, vm::LoxVM};


#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Group (Box<Expr>),
    Literal (Token),
    Unary (Token, Box<Expr>),
}


impl Expr {

    pub fn evaluate(&self, vm: &mut LoxVM) -> Result<Object, String> {
        use Expr::*;
        use Token::{*};
        match self {
            // simple values
            Literal(Nil) => Ok(Object::new()),
            Literal(False) => Ok(Object::bool_new(false)),
            Literal(True) => Ok(Object::bool_new(true)),
            Literal(String(str)) => Ok(Object::string_new(str.clone())),
            Literal(Number(num)) => Ok(Object::number_new(num.clone())),
            Literal(Identifier(idnt_name)) => {
                match vm.var_get(idnt_name) {
                    Some(oo) => Ok(oo.clone()),
                    None => Err(format!("object {} not found", idnt_name)),
                }
            },
            // Unary expr
            Unary(Bang, expr) => expr.evaluate(vm)?.not(),
            Unary(Minus, expr) => expr.evaluate(vm)?.neg(),
            // Group expr
            Group(expr) => expr.evaluate(vm),
            // Binary
            Binary(left, Slash, right) => left.evaluate(vm)?.div(&right.evaluate(vm)?),
            Binary(left, Star, right) => left.evaluate(vm)?.mul(&right.evaluate(vm)?),
            Binary(left, Minus, right) => left.evaluate(vm)?.sub(&right.evaluate(vm)?),
            Binary(left, Plus, right) => left.evaluate(vm)?.add(&right.evaluate(vm)?),
            Binary(left, Greater, right) => left.evaluate(vm)?.gt(&right.evaluate(vm)?),
            Binary(left, GreaterEqual, right) => left.evaluate(vm)?.ge(&right.evaluate(vm)?),
            Binary(left, Less, right) => left.evaluate(vm)?.lt(&right.evaluate(vm)?),
            Binary(left, LessEqual, right) => left.evaluate(vm)?.le(&right.evaluate(vm)?),
            Binary(left, EqualEqual, right) => left.evaluate(vm)?.eq(&right.evaluate(vm)?),
            Binary(left, BangEqual, right) => left.evaluate(vm)?.ne(&right.evaluate(vm)?),
            left => {
                Err(format!("NOT CHECKED TYPE: {:#?}", left))
            },
        }
    }

}

// parsing methods
impl Expr {
    pub fn expression(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        Self::equality(tks, start)
    }

    pub fn equality(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        let (mut expr, adv) = Self::comparison(tks, start)?;
        let mut addup_adv = adv;

        while let tk_op @ Some(Token::EqualEqual | Token::BangEqual) = tks.get(start + addup_adv) {
            match Self::comparison(tks, start + addup_adv + 1) {
                Err(_) => break,
                Ok((right, adv)) => {
                    expr = Expr::Binary(Box::new(expr), tk_op.unwrap().clone(), Box::new(right));
                    addup_adv += adv + 1;     // operation token                    
                },
            }
        }

        Ok((expr, addup_adv))
    }

    pub fn comparison(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        let (mut expr, adv) = Self::term(tks, start)?;
        let mut addup_adv = adv;

        while let tk_op @ Some(Token::Greater | Token::GreaterEqual 
                        | Token::Less | Token::LessEqual) = tks.get(start + addup_adv) {
            match Self::term(tks, start + addup_adv + 1) {
                Err(_) => break,
                Ok((right, adv)) => {
                    expr = Expr::Binary(Box::new(expr), tk_op.unwrap().clone(), Box::new(right));
                    addup_adv += adv + 1;     // operation token                    
                },
            }
        }

        Ok((expr, addup_adv))
    }

    pub fn term(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        let (mut expr, adv) = Self::factor(tks, start)?;
        let mut addup_adv = adv;

        while let tk_op @ Some(Token::Minus | Token::Plus) = tks.get(start + addup_adv) {
            match Self::factor(tks, start + addup_adv + 1) {
                Err(_) => break,
                Ok((right, adv)) => {
                    expr = Expr::Binary(Box::new(expr), tk_op.unwrap().clone(), Box::new(right));
                    addup_adv += adv + 1;     // operation token                    
                },
            }
        }

        Ok((expr, addup_adv))
    }

    pub fn factor(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        let (mut expr, adv) = Self::unary(tks, start)?;
        let mut addup_adv = adv;

        while let tk_op @ Some(Token::Slash | Token::Star) = tks.get(start + addup_adv) {
            match Self::unary(tks, start + addup_adv + 1) {
                Err(_) => break,
                Ok((right, adv)) => {
                    expr = Expr::Binary(Box::new(expr), tk_op.unwrap().clone(), Box::new(right));
                    addup_adv += adv + 1;     // operation token                    
                },
            }
        }

        Ok((expr, addup_adv))
    }

    pub fn unary(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        match tks.get(start) {
            tk_op @ Some(Token::Bang | Token::Minus) => {
                let (expr, adv) = Self::unary(tks, start+1)?;
                Ok((Expr::Unary(tk_op.unwrap().clone(), Box::new(expr)), 1+adv))      // !/- + 
            },
            _ => {
                Self::primary(tks, start)
            },
        }
    }

    pub fn primary(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        match tks.get(start) {
            tk @ Some(Token::False | Token::True | Token::Nil) => Ok((Expr::Literal(tk.unwrap().clone()), 1)),
            tk @ Some(Token::String(_) | Token::Number(_)) => Ok((Expr::Literal(tk.unwrap().clone()), 1)),
            tk @ Some(Token::Identifier(_)) => Ok((Expr::Literal(tk.unwrap().clone()), 1)),
            Some(Token::LeftParen) => {
                let (expr, adv) = Self::expression(tks, start+1)?;

                if matches!(tks.get(start+adv+1), Some(Token::RightParen)) {
                    Ok((Expr::Group(Box::new(expr)), 2 + adv))  // L/R Paren + increased index(adv)
                } else {
                    Err(format!("cannot get close paren"))
                }
            },
            tk => Err(format!("unexpected token/status: {:#?}", tk)),
        }
    }

    pub fn synchronize(tks: &Vec<Token>, start: usize) -> usize {
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



