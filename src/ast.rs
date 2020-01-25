pub enum Operator {
    Plus,
    Minus,
    Star,
    // Divide

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual
}

pub enum Expr {
    Binary(Box<Expr>, Box<Expr>, Operator),
    Grouping(Box<Expr>),
    Literal(String), // TODO: Change this to primitive?
    Unary(Operator, Expr),
}