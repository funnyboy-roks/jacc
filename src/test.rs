macro_rules! var {
    ($name: ident) => {
        Box::new($crate::ast::AstStatement::Variable(
            stringify!($name).into(),
        ))
    };
}

macro_rules! num {
    ($n: literal) => {
        Box::new($crate::ast::AstStatement::Number($n))
    };
}

macro_rules! op {
    ($o: ident) => {
        Box::new($crate::ast::AstStatement::Operator(
            $crate::ast::op::Operator::$o,
        ))
    };
}

macro_rules! expr {
    ($($a: expr),*) => {
        Box::new($crate::ast::AstStatement::InfixExpression(vec![$($a),*]))
    }
}

macro_rules! fun {
    ($name: ident ($($a: expr),*)) => {
        Box::new($crate::ast::AstStatement::FunctionCall {
            name: stringify!($name).into(),
            params: vec![$($a),*],
        })
    };
}
