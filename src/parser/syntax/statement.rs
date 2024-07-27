use crate::parser::{types::object::Object, LoxParser};

use super::{expression::Expr, token::Token};



#[derive(Debug)]
pub enum Stmt{
    Block(Vec<Stmt>),
    Decl(Token, Option<Expr>),
    Expr(Expr),
    Print(Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
}


impl Stmt {

    pub fn exec(&self, parser: &mut LoxParser) -> Result<Option<i32>, String> {
        match self {
            Stmt::Expr(expr) => {
                println!("{}", expr.evaluate(parser)?);
            },
            Stmt::Print(expr) => {
                println!("{}", expr.evaluate(parser)?);
            },
            Stmt::Block(stmts) => {
                parser.vm.stack_current().scope_enter();
                let mut iter = stmts.iter();
                while let Some(stmt) = iter.next() {
                    stmt.exec(parser)?;
                }
                parser.vm.stack_current().scope_exit();
            }
            Stmt::If(cont, stmt_true, opt_false) => {
                if cont.evaluate(parser)?.is_true()? {
                    stmt_true.exec(parser)?;
                } else if let Some(stmt_false) = opt_false {
                    stmt_false.exec(parser)?;
                }
            },
            Stmt::Decl(Token::Identifier(idnt_name), expr) => {
                match expr {
                    Some(expr) => {
                        let obj = expr.evaluate(parser)?;
                        parser.vm.var_add(idnt_name.clone(), obj);
                    },
                    _ => {
                        parser.vm.var_add(idnt_name.clone(), Object::Nil);
                    },
                };
            },
            _ => {
                return Err("Unexpected statement".to_string());
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
            Some(Token::If) => Ok(Self::ctrl_if(tks, start)?),
            Some(Token::LeftBrace) => Ok(Self::block(tks, start)?),
            Some(_) => Self::expr(tks, start),
            None => Err("Failed to get token from list".to_string()),
        }
    }

    pub fn ctrl_if(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        if !matches!(tks.get(start), Some(Token::If)) {
            return Err("not start with Token: if".to_string());
        }
        let mut ret_adv = 1;
        match tks.get(start + ret_adv) {
            Some(Token::LeftParen) => ret_adv += 1,
            tk => return Err(format!("expected (, but got {:#?}", tk)),
        }

        let (expr_cont, used) = Expr::expression(tks, start+ret_adv)?;
        ret_adv += used;

        match tks.get(start + ret_adv) {
            Some(Token::RightParen) => ret_adv += 1,
            tk => return Err(format!("expected ), but got {:#?}", tk)),
        }

        let (stmt_true, used) = Self::stmt(tks, start + ret_adv)?;
        ret_adv += used;

        let mut opt_false = None;
        if let Some(Token::Else) = tks.get(start+ret_adv) {
            let (stmt_false, used) = Self::stmt(tks, start+ret_adv+1)?;
            ret_adv += used + 1;
            opt_false = Some(Box::new(stmt_false));
        }
        Ok((Stmt::If(expr_cont, Box::new(stmt_true), opt_false), ret_adv))

    }

    pub fn block(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        if !matches!(tks.get(start), Some(Token::LeftBrace)) {
            return Err("not start with Token: {".to_string());
        }
        let mut ret_adv = 1;
        let mut stmt_arr = Vec::new();

        loop {
            if let Some(Token::RightBrace) = tks.get(start+ ret_adv) {
                ret_adv += 1;
                break;
            }
            let (stmt, used) = Self::stmt(tks, start + ret_adv)?;
            stmt_arr.push(stmt);
            ret_adv += used;
        }

        Ok((Self::Block(stmt_arr), ret_adv))
    }

    pub fn decl(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {

        if !matches!(tks.get(start), Some(Token::Var)) {
            return Err("not start with Token: Var".to_string());
        }
        let mut ret_adv = 1;
        let idnt = tks.get(start + ret_adv);
        if matches!(idnt, None) {
            return Err("cannot get more tokens".to_string());
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
                            _ => Err("failed to parse statement".to_string()),
                        }
                    },
                    _ => Err("failed to parse expression".to_string()),
                }
            },
            tk => return Err(format!("unexpected token: {:#?}", tk)),
        }

    }

    pub fn expr(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        let (expr, adv) = Expr::expression(tks, start)?;

        match tks.get(start + adv) {
            Some(Token::Semicolon) => Ok((Stmt::Expr(expr), adv+1)),
            tk => Err(format!("unexpected token: {:#?}", tk)),
        }
    }

    pub fn print(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        if !matches!(tks.get(start), Some(Token::Print)) {
            return Err("not start with Token: Print".to_string());
        }
        let mut used_adv = 1;

        let (expr, adv) = Expr::expression(tks, start + used_adv)?;
        used_adv += adv;

        match tks.get(start + used_adv) {
            Some(Token::Semicolon) => Ok((Stmt::Print(expr), used_adv+1)),
            tk => Err(format!("unexpected token: {:#?}", tk)),
        }

    }
}
