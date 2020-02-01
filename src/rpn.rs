use crate::ast::Expr;

fn rpn(expr: &Expr) -> String {
    match expr {
        Expr::Binary(o, b1, b2) => format!("{} {} {}", rpn(b1), rpn(b2), o),
        Expr::Grouping(b) => format!("{}", rpn(b)),
        Expr::Literal(p) => format!("{}", p),
        Expr::Unary(_o, _b) => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Expr, Operator, Primitive};

    #[test]
    fn test_rpn() {
        let expr = Expr::Binary(
            Operator::Star,
            Box::new(Expr::Grouping(Box::new(Expr::Binary(
                Operator::Plus,
                Box::new(Expr::Literal(Primitive::Number(1.0))),
                Box::new(Expr::Literal(Primitive::Number(2.0))),
            )))),
            Box::new(Expr::Grouping(Box::new(Expr::Binary(
                Operator::Minus,
                Box::new(Expr::Literal(Primitive::Number(4.0))),
                Box::new(Expr::Literal(Primitive::Number(3.0))),
            )))),
        );
        assert_eq!(rpn(&expr), "1 2 + 4 3 - *")
    }
}
