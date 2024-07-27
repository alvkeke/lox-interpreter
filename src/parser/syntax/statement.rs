use crate::{dbg_format, parser::{types::object::Object, LoxParser}};

use super::{expression::Expr, token::Token};



#[derive(Debug)]
pub enum Stmt{
    Block(Vec<Stmt>),
    Decl(Token, Option<Expr>),
    Expr(Expr),
    Print(Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    For(Option<Box<Stmt>>, Option<Expr>, Option<Expr>, Box<Stmt>),
}


impl Stmt {

    pub fn exec(&self, parser: &mut LoxParser) -> Result<Option<i32>, String> {
        match self {
            Stmt::Expr(expr) => {
                expr.evaluate(parser)?;
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
            Stmt::While(cont, body) => {
                while cont.evaluate(parser)?.is_true()? {
                    body.exec(parser)?;
                }
            },
            Stmt::For(start, cont, every, body) => {
                if let Some(start) = start {
                    start.exec(parser)?;
                }
                loop {
                    if let Some(cont) = cont {
                        if !cont.evaluate(parser)?.is_true()? {
                            break;
                        }
                    }
                    body.exec(parser)?;

                    if let Some(every) = every {
                        every.evaluate(parser)?;
                    }
                }
            },
            _ => {
                return Err(dbg_format!("Unexpected statement"));
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
            Some(Token::While) => Ok(Self::ctrl_while(tks, start)?),
            Some(Token::For) => Ok(Self::ctrl_for(tks, start)?),
            Some(Token::LeftBrace) => Ok(Self::block(tks, start)?),
            Some(_) => Self::expr(tks, start),
            None => Err(dbg_format!("Failed to get token from list")),
        }
    }

    pub fn ctrl_for(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        if !matches!(tks.get(start), Some(Token::For)) {
            return Err(dbg_format!("not start with Token: while"));
        }
        let mut ret_adv = 1;
        match tks.get(start + ret_adv) {
            Some(Token::LeftParen) => ret_adv += 1,
            tk => return Err(dbg_format!("expected (, but got {:#?}", tk)),
        }
        let opt_start;
        match tks.get(start+ret_adv) {
            Some(Token::Semicolon) => {
                opt_start = None;
                ret_adv += 1;
            },
            Some(Token::Var) => {
                let (stmt, used) = Self::decl(tks, start+ret_adv)?;
                opt_start = Some(Box::new(stmt));
                ret_adv += used;
            },
            _ => {
                let (stmt, used) = Self::expr(tks, start+ret_adv)?;
                opt_start = Some(Box::new(stmt));
                ret_adv += used;
            },
        }

        let opt_cont;
        match Expr::expression(tks, start+ret_adv) {
            Ok((cont, used)) => {
                opt_cont = Some(cont);
                ret_adv += used;
            },
            _ => opt_cont = None,
        }
        match tks.get(start + ret_adv) {
            Some(Token::Semicolon) => ret_adv += 1,
            tk => return Err(dbg_format!("expected ;, but got {:#?}", tk)),
        }

        let opt_every;
        match Expr::expression(tks, start+ret_adv) {
            Ok((cont, used)) => {
                opt_every = Some(cont);
                ret_adv += used;
            },
            _ => opt_every = None,
        }
        match tks.get(start + ret_adv) {
            Some(Token::RightParen) => ret_adv += 1,
            tk => return Err(dbg_format!("expected ), but got {:#?}", tk)),
        };
        let (stmt_body, used) = Self::stmt(tks, start + ret_adv)?;
        ret_adv += used;

        Ok((Stmt::For(opt_start, opt_cont, opt_every, Box::new(stmt_body)), ret_adv))
    }

    pub fn ctrl_while(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        if !matches!(tks.get(start), Some(Token::While)) {
            return Err(dbg_format!("not start with Token: while"));
        }
        let mut ret_adv = 1;
        match tks.get(start + ret_adv) {
            Some(Token::LeftParen) => ret_adv += 1,
            tk => return Err(dbg_format!("expected (, but got {:#?}", tk)),
        }

        let (expr_cont, used) = Expr::expression(tks, start+ret_adv)?;
        ret_adv += used;

        match tks.get(start + ret_adv) {
            Some(Token::RightParen) => ret_adv += 1,
            tk => return Err(dbg_format!("expected ), but got {:#?}", tk)),
        };
        let (stmt_true, used) = Self::stmt(tks, start + ret_adv)?;
        ret_adv += used;

        Ok((Stmt::While(expr_cont, Box::new(stmt_true)), ret_adv))
    }

    pub fn ctrl_if(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        if !matches!(tks.get(start), Some(Token::If)) {
            return Err(dbg_format!("not start with Token: if"));
        }
        let mut ret_adv = 1;
        match tks.get(start + ret_adv) {
            Some(Token::LeftParen) => ret_adv += 1,
            tk => return Err(dbg_format!("expected (, but got {:#?}", tk)),
        }

        let (expr_cont, used) = Expr::expression(tks, start+ret_adv)?;
        ret_adv += used;

        match tks.get(start + ret_adv) {
            Some(Token::RightParen) => ret_adv += 1,
            tk => return Err(dbg_format!("expected ), but got {:#?}", tk)),
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
            return Err(dbg_format!("not start with Token: {"));
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
            return Err(dbg_format!("not start with Token: Var"));
        }
        let mut ret_adv = 1;
        let idnt = tks.get(start + ret_adv);
        if matches!(idnt, None) {
            return Err(dbg_format!("cannot get more tokens"));
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
                            _ => Err(dbg_format!("failed to parse statement")),
                        }
                    },
                    _ => Err(dbg_format!("failed to parse expression")),
                }
            },
            tk => return Err(dbg_format!("unexpected token: {:#?}", tk)),
        }

    }

    pub fn expr(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        let (expr, adv) = Expr::expression(tks, start)?;

        match tks.get(start + adv) {
            Some(Token::Semicolon) => Ok((Stmt::Expr(expr), adv+1)),
            tk => Err(dbg_format!("unexpected token: {:#?}", tk)),
        }
    }

    pub fn print(tks: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        if !matches!(tks.get(start), Some(Token::Print)) {
            return Err(dbg_format!("not start with Token: Print"));
        }
        let mut used_adv = 1;

        let (expr, adv) = Expr::expression(tks, start + used_adv)?;
        used_adv += adv;

        match tks.get(start + used_adv) {
            Some(Token::Semicolon) => Ok((Stmt::Print(expr), used_adv+1)),
            tk => Err(dbg_format!("unexpected token: {:#?}", tk)),
        }

    }
}
