use crate::token::Token;


struct Object {

}

struct Expr {

}

pub enum Syntax {
    Object(Object),
    Expr(Expr),
    Binary {
        left: Expr, 
        op: Token, 
        right: Expr,
    },
    Group {
        expr: Expr,
    },
    Literal {
        value: Object
    },
    Unary {
        op: Token,
        right: Expr,
    },

}



