use crate::{types::object::Object, vm::LoxVM};

use super::{expression::Expr, token::Token};



#[derive(Debug)]
pub enum Stmt{
    Decl(Token, Option<Expr>),
    Expr(Expr),
    Print(Expr),
}


impl Stmt {

    pub fn exec(&self, vm: &mut LoxVM) -> Result<Option<Object>, String> {
        match self {
            Stmt::Expr(expr) => {
                println!("{}", expr.evaluate(vm)?);
            },
            Stmt::Print(expr) => {
                println!("{}", expr.evaluate(vm)?);
            },
            Stmt::Decl(Token::Identifier(idnt_name), expr) => {
                match expr {
                    Some(expr) => {
                        let mut obj = expr.evaluate(vm)?;
                        obj.set_name(idnt_name.clone());
                        return Ok(Some(obj));
                    },
                    _ => {
                        let mut obj = Object::new();
                        obj.set_name(idnt_name.clone());
                        return Ok(Some(obj));
                    },
                }
            },
            _ => {
                return Err(format!("Unexpected statement"));
            },
        }
        Ok(None)
    }

}


impl Stmt {

    pub fn stmt(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        match tks.get(start) {
            Some(Token::Print) => {
                let (stmt, used) = Self::print(tks, start)?;
                Ok((stmt, used))
            },
            Some(Token::Var) => {
                let (stmt, used) = Self::decl(tks, start)?;
                Ok((stmt, used))
            },
            Some(_) => Self::expr(tks, start),
            None => Err(format!("Failed to get token from list")),
        }
    }

    pub fn decl(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {

        if !matches!(tks.get(start), Some(Token::Var)) {
            return Err(format!("not start with Token: Var"));
        }
        let mut ret_adv = 1;
        let idnt = tks.get(start + ret_adv);
        if matches!(idnt, None) {
            return Err(format!("cannot get more tokens"));
        }
        let idnt = idnt.unwrap().clone();
        ret_adv += 1;
        
        match tks.get(start + ret_adv) {
            Some(Token::Semicolon) => {
                // just return if end with `;'
                ret_adv += 1;
                return Ok((Stmt::Decl(idnt, None), ret_adv));
            },
            Some(Token::Equal) => {
                ret_adv += 1;
                match Expr::expression(tks, start + ret_adv) {
                    Ok((expr, adv)) => {
                        ret_adv += adv;
                        match tks.get(start + ret_adv) {
                            Some(Token::Semicolon) => Ok((Stmt::Decl(idnt, Some(expr)), ret_adv+1)),
                            _ => Err(format!("failed to parse statement")),
                        }
                    },
                    _ => Err(format!("failed to parse expression")),
                }
            },
            tk => return Err(format!("unexpected token: {:#?}", tk)),
        }

    }

    pub fn expr(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        let (expr, adv) = Expr::expression(tks, start)?;

        match tks.get(start + adv) {
            Some(Token::Semicolon) => Ok((Stmt::Expr(expr), adv+1)),
            tk => Err(format!("unexcepted token: {:#?}", tk)),
        }
    }

    pub fn print(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        if !matches!(tks.get(start), Some(Token::Print)) {
            return Err(format!("not start with Token: Print"));
        }
        let mut used_adv = 1;

        let (expr, adv) = Expr::expression(tks, start + used_adv)?;
        used_adv += adv;

        match tks.get(start + used_adv) {
            Some(Token::Semicolon) => Ok((Stmt::Print(expr), used_adv+1)),
            tk => Err(format!("unexcepted token: {:#?}", tk)),
        }

    }
}
