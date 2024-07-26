use crate::parser::{syntax::token::Token, types::object::Object, LoxParser};


#[derive(Debug)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Group (Box<Expr>),
    Literal (Token),
    Unary (Token, Box<Expr>),
}


impl Expr {

    pub fn evaluate(&self, parser: &mut LoxParser) -> Result<Object, String> {
        use Expr::*;
        use Token::{*};
        match self {
            // simple values
            Literal(Nil) => Ok(Object::Nil),
            Literal(False) => Ok(Object::Boolean(false)),
            Literal(True) => Ok(Object::Boolean(true)),
            Literal(String(str)) => Ok(Object::String(str.clone())),
            Literal(Number(num)) => Ok(Object::Number(num.clone())),
            Literal(Identifier(idnt_name)) => {
                match parser.vm.auto_obj_get(idnt_name) {
                    Some(oo) => Ok(oo.clone()),
                    None => Err(format!("object {} not found", idnt_name)),
                }
            },
            // Unary expr
            Unary(Bang, expr) => expr.evaluate(parser)?.not(),
            Unary(Minus, expr) => expr.evaluate(parser)?.neg(),
            // Group expr
            Group(expr) => expr.evaluate(parser),
            // Binary
            Binary(left, Slash, right) => left.evaluate(parser)?.div(&right.evaluate(parser)?),
            Binary(left, Star, right) => left.evaluate(parser)?.mul(&right.evaluate(parser)?),
            Binary(left, Minus, right) => left.evaluate(parser)?.sub(&right.evaluate(parser)?),
            Binary(left, Plus, right) => left.evaluate(parser)?.add(&right.evaluate(parser)?),
            Binary(left, Greater, right) => left.evaluate(parser)?.gt(&right.evaluate(parser)?),
            Binary(left, GreaterEqual, right) => left.evaluate(parser)?.ge(&right.evaluate(parser)?),
            Binary(left, Less, right) => left.evaluate(parser)?.lt(&right.evaluate(parser)?),
            Binary(left, LessEqual, right) => left.evaluate(parser)?.le(&right.evaluate(parser)?),
            Binary(left, EqualEqual, right) => left.evaluate(parser)?.eq(&right.evaluate(parser)?),
            Binary(left, BangEqual, right) => left.evaluate(parser)?.ne(&right.evaluate(parser)?),
            Assign(Identifier(idnt_name), expr) => {
                let value = expr.evaluate(parser)?;
                parser.vm.auto_obj_set_if_exist(idnt_name.clone(), value)
            },
            left => {
                Err(format!("NOT CHECKED TYPE: {:#?}", left))
            },
        }
    }

}

// parsing methods
impl Expr {
    pub fn expression(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        if let Ok(ret) = Self::assign(tks, start) {
            Ok(ret)
        } else {
            Self::equality(tks, start)
        }
    }

    pub fn assign(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        let mut ret_adv = 0;
        let idt = tks.get(start+ret_adv);
        if !matches!(idt, Some(Token::Identifier(_))) {
            return Err("Not start with Identifier".to_string());
        }
        let idt = idt.unwrap();
        ret_adv+=1;
        if !matches!(tks.get(start+ ret_adv), Some(Token::Equal)) {
            return Err("missing token : Equal".to_string());
        }
        ret_adv+=1;

        match Self::assign(tks, start + ret_adv) {
            Ok((expr, used)) => {
                ret_adv += used;
                Ok((Expr::Assign(idt.clone(), Box::new(expr)), ret_adv))
            },
            Err(_) => {
                match Self::equality(tks, start+ret_adv) {
                    Ok((expr, used)) => {
                        ret_adv += used;
                        Ok((Expr::Assign(idt.clone(), Box::new(expr)), ret_adv))
                    },
                    Err(msg) => Err(msg),
                }
            }
        }
    }

    fn binary_common(
        tks: &Vec<Token>, start: usize,
        next_fn: fn(&Vec<Token>, usize) -> Result<(Self, usize), String>,
        ops: &[Token]
    ) -> Result<(Self, usize), String> {
        let (mut expr, adv) = next_fn(tks, start)?;
        let mut ret_adv = adv;

        while let Some(tk_op) = tks.get(start + ret_adv) {
            if ops.contains(tk_op) {
                match next_fn(tks, start + ret_adv + 1) {
                    Err(_) => break,
                    Ok((right, adv)) => {
                        expr = Expr::Binary(Box::new(expr), tk_op.clone(), Box::new(right));
                        ret_adv += adv + 1; // operation token
                    },
                }
            } else {
                break;
            }
        }

        Ok((expr, ret_adv))
    }

    const EQUALITY_OPS: [Token; 2] = [Token::EqualEqual, Token::BangEqual];
    pub fn equality(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        Self::binary_common(tks, start, Self::comparison, &Self::EQUALITY_OPS)
    }

    const COMPARISON_OPS: [Token; 4] = [Token::Greater, Token::GreaterEqual, Token::Less, Token::LessEqual];
    pub fn comparison(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        Self::binary_common(tks, start, Self::term, &Self::COMPARISON_OPS)
    }
    const TERM_OPS: [Token; 2] = [Token::Minus, Token::Plus];
    pub fn term(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        Self::binary_common(tks, start, Self::factor, &Self::TERM_OPS)
    }

    const FACTOR_OPS: [Token; 2] = [Token::Slash, Token::Star];
    pub fn factor(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        Self::binary_common(tks, start, Self::unary, &Self::FACTOR_OPS)
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
                    Err("cannot get close paren".to_string())
                }
            },
            tk => Err(format!("unexpected token/status: {:#?}", tk)),
        }
    }

    #[allow(dead_code)]
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



