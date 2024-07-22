
use crate::token::Token;


struct Object;

struct Expr;

struct Binary {
    left: Expr,
    operator: Token, 
    right: Expr,
}

struct Group {
    expression: Expr,
}

struct Literal {
    value: Object,
}

struct Unary {
    operator: Token,
    right: Expr,
}

impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Binary{
        Binary{left, operator, right}
    }
}

impl Group {
    pub fn new(expression: Expr) -> Group {
        Group{expression}
    }
}


impl Literal {
    pub fn new(value: Object) -> Literal {
        Literal { value: value }
    }
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Unary {
        Unary{operator, right}
    }
}
