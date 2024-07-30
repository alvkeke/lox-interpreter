use crate::{dbg_format, parser::{syntax::token::Token, types::object::Object, LoxParser}};


#[derive(Debug)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Group (Box<Expr>),
    Literal (Token),
    Unary (Token, Box<Expr>),
}


impl Expr {

    pub fn evaluate(&self, vm: &mut LoxParser) -> Result<Object, String> {
        use Expr::*;
        use Token::{*};
        match self {
            // simple values
            Literal(Nil) => Ok(Object::Nil),
            Literal(False) => Ok(Object::Boolean(false)),
            Literal(True) => Ok(Object::Boolean(true)),
            Literal(String(str)) => Ok(Object::String(str.clone())),
            Literal(Number(num)) => Ok(Object::Number(num.clone())),
            Literal(Identifier(idnt_name)) => Ok(vm.var_get(idnt_name)?.clone()),
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
            Binary(left, And, right) => left.evaluate(vm)?.logic_and(&right.evaluate(vm)?),
            Binary(left, Or, right) => left.evaluate(vm)?.logic_or(&right.evaluate(vm)?),
            Assign(Identifier(idnt_name), expr) => {
                let value = expr.evaluate(vm)?;
                vm.var_set(idnt_name.clone(), value)
            },
            left => {
                Err(dbg_format!("NOT CHECKED TYPE: {:#?}", left))
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
            Self::logic_or(tks, start)
        }
    }

    pub fn assign(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        let mut ret_adv = 0;
        let idt = tks.get(start+ret_adv);
        if !matches!(idt, Some(Token::Identifier(_))) {
            return Err(dbg_format!("Not start with Identifier"));
        }
        let idt = idt.unwrap();
        ret_adv+=1;
        if !matches!(tks.get(start+ ret_adv), Some(Token::Equal)) {
            return Err(dbg_format!("missing token : Equal"));
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

    const LOGIC_OR_OPS: [Token; 1] = [Token::Or];
    pub fn logic_or(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        Self::binary_common(tks, start, Self::logic_and, &Self::LOGIC_OR_OPS)
    }

    const LOGIC_AND_OPS: [Token; 1] = [Token::And];
    pub fn logic_and(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        Self::binary_common(tks, start, Self::equality, &Self::LOGIC_AND_OPS)
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
                    Err(dbg_format!("cannot get close paren"))
                }
            },
            Some(tk) => Err(dbg_format!("unexpected token: {:#?}", tk)),
            None => Err(dbg_format!("unexpected status")),
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



