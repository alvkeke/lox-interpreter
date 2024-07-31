use crate::{dbg_format, parser::syntax::token::Token};


#[derive(Debug)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Group (Box<Expr>),
    Literal (Token),
    Unary (Token, Box<Expr>),
    FnCall(String, Vec<Box<Expr>>),
}

impl Clone for Expr {
    fn clone(&self) -> Self {
        match self {
            Self::Assign(arg0, arg1) => Self::Assign(arg0.clone(), arg1.clone()),
            Self::Binary(arg0, arg1, arg2) => Self::Binary(arg0.clone(), arg1.clone(), arg2.clone()),
            Self::Group(arg0) => Self::Group(arg0.clone()),
            Self::Literal(arg0) => Self::Literal(arg0.clone()),
            Self::Unary(arg0, arg1) => Self::Unary(arg0.clone(), arg1.clone()),
            Self::FnCall(arg0, arg1) => Self::FnCall(arg0.clone(), arg1.clone()),
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

        let tk_start = tks.get(start);
        if let tk_op@ Some(Token::Bang | Token::Minus) = tk_start {
            let (expr, adv) = Self::unary(tks, start+1)?;
            return Ok((Expr::Unary(tk_op.unwrap().clone(), Box::new(expr)), 1+adv));      // !/- +
        }

        if let Some(Token::Identifier(_)) = tk_start {
            if let Ok((expr, used)) = Self::fn_call(tks, start) {
                return Ok((expr, used));
            }
        }

        Self::primary(tks, start)
    }

    fn fn_args_parse(tks: &Vec<Token>, start: usize) -> Result<(Vec<Box<Self>>, usize), String> {
        let mut ret_adv = 0;
        match tks.get(start+ret_adv) {
            Some(Token::LeftParen) => ret_adv += 1,
            tk => return Err(dbg_format!("expected token (, but got {:#?}", tk)),
        }

        let mut args = Vec::new();

        if let Ok((expr, used)) = Self::expression(tks, start+ ret_adv) {
            args.push(Box::new(expr));
            ret_adv += used;
            while let Some(Token::Comma) = tks.get(start+ret_adv) {
                if args.len() >= 255 {
                    return Err(dbg_format!("Cannot have more then 255 arguments."));
                }
                ret_adv += 1;
                let (expr, used) = Self::expression(tks, start+ret_adv)?;
                args.push(Box::new(expr));
                ret_adv += used;
            }
        }

        match tks.get(start+ret_adv) {
            Some(Token::RightParen) => ret_adv += 1,
            tk => return Err(dbg_format!("expected token ), but got {:#?}", tk)),
        }

        Ok((args, ret_adv))
    }

    pub fn fn_call(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        let mut ret_adv = 0;
        let fn_name;
        match tks.get(start) {
            Some(Token::Identifier(idnt)) => {
                ret_adv += 1;
                fn_name = idnt.clone();
            },
            _ => return Err(dbg_format!("not an valid function call")),
        }

        let (args, used) = Self::fn_args_parse(tks, start+ret_adv)?;
        ret_adv += used;

        Ok((Expr::FnCall(fn_name, args), ret_adv))
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



