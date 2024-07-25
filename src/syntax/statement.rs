use crate::types::object::Object;

use super::{expression::Expr, token::Token};



#[derive(Debug)]
pub enum Stmt{
    Decl(Token, Option<Expr>),
    Expr(Expr),
    Print(Expr),
    None,
}


impl Stmt {

    pub fn visit(self)  -> Result<(), String> {
        self.exec()
    }

    pub fn exec(&self) -> Result<(), String> {
        match self {
            Stmt::Expr(expr) => {
                println!("{}", expr.evaluate()?);
            },
            Stmt::Print(expr) => {
                print!("{}", expr.evaluate()?);
            },
            Stmt::Decl(Token::Identifier(idnt_name), expr) => {
                println!("TODO::: decleared var: {} = {:?}", idnt_name, expr);
                // todo!()
            },
            _ => {
                return Err(format!("Unexpected statement"));
            },
        }
        Ok(())
    }

}


impl Stmt {

    pub fn stmt(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        match tks.get(start) {
            Some(Token::Print) => Self::print(tks, start),
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
        
        if !matches!(tks.get(start+ret_adv), Some(Token::Equal)) {
            return Ok((Stmt::Decl(idnt, None), ret_adv));
        }

        match Expr::expression(tks, start + ret_adv + 1) {
            Ok((expr, adv)) => {
                ret_adv += adv;
                match tks.get(start + ret_adv) {
                    Some(Token::Semicolon) => Ok((Stmt::Decl(idnt, Some(expr)), ret_adv+1)),
                    _ => Err(format!("failed to parse statement")),
                }
            },
            _ => Err(format!("failed to parse expression")),
        }

    }

    pub fn expr(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        let (expr, adv) = Expr::expression(tks, start)?;

        match tks.get(start + adv) {
            Some(Token::Semicolon) => Ok((Stmt::Expr(expr), adv+1)),
            tk => Err(format!("unexcepted token: {:?}", tk)),
        }
    }

    pub fn print(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        if !matches!(tks.get(start), Some(Token::Print)) {
            return Err(format!("not start with Token: Print"));
        }

        let (expr, adv) = Expr::expression(tks, start + 1)?;

        match tks.get(start + 1 + adv) {
            Some(Token::Semicolon) => Ok((Stmt::Print(expr), adv+1)),
            tk => Err(format!("unexcepted token: {:?}", tk)),
        }

    }
}
