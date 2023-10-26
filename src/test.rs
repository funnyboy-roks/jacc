use super::ast::*;

macro_rules! var {
    ($name: ident) => {
        $crate::ast::AstStatement::Variable(stringify!($name).into())
    };
}

macro_rules! num {
    ($n: literal) => {
        $crate::ast::AstStatement::Number($n)
    };
}

macro_rules! op {
    ($o: ident) => {
        crate::ast::AstStatement::Operator($crate::ast::op::Operator::$o)
    };
}

macro_rules! expr {
    ($($a: expr),*) => {
        crate::ast::AstStatement::InfixExpression(vec![$($a),*])
    }
}

macro_rules! fun {
    ($name: ident ($($a: expr),*)) => {
        crate::ast::AstStatement::FunctionCall {
            name: stringify!($name).into(),
            params: vec![$($a),*],
        }
    };
}

#[test]
fn infix_postfix() {
    let eval = AstEvaluator::new();
    // 1 * (2 + 3)
    let statement = vec![
        num!(1.0),
        op!(Multiply),
        op!(LeftParen),
        num!(2.0),
        op!(Add),
        num!(3.0),
        op!(RightParen),
    ];

    let pf = AstEvaluator::infix_to_postfix(&statement).unwrap();

    assert_eq!(
        pf,
        vec![num!(1.0), num!(2.0), num!(3.0), op!(Add), op!(Multiply),]
    );

    let res = eval
        .eval(&AstStatement::InfixExpression(statement))
        .unwrap();

    assert_eq!(res, 5.0);
}

#[test]
fn infix_postfix_hard() {
    let eval = AstEvaluator::new();
    // 1 + 2 * ( 3 ** 4 - 5 ) ** ( 6 + 7 * 8 ) - 9
    let statement = vec![
        num!(1.0),
        op!(Add),
        num!(2.0),
        op!(Multiply),
        op!(LeftParen),
        num!(3.0),
        op!(Exponent),
        num!(4.0),
        op!(Subtract),
        num!(5.0),
        op!(RightParen),
        op!(Exponent),
        op!(LeftParen),
        num!(6.0),
        op!(Add),
        num!(7.0),
        op!(Multiply),
        num!(8.0),
        op!(RightParen),
        op!(Subtract),
        num!(9.0),
    ];

    let pf = AstEvaluator::infix_to_postfix(&statement).unwrap();

    // 1 2 3 4 ** 5 - 6 7 8 * + ** * + 9 -
    let expected = vec![
        num!(1.0),
        num!(2.0),
        num!(3.0),
        num!(4.0),
        op!(Exponent),
        num!(5.0),
        op!(Subtract),
        num!(6.0),
        num!(7.0),
        num!(8.0),
        op!(Multiply),
        op!(Add),
        op!(Exponent),
        op!(Multiply),
        op!(Add),
        num!(9.0),
        op!(Subtract),
    ];
    assert_eq!(pf, expected);

    let res = eval
        .eval(&AstStatement::InfixExpression(statement))
        .unwrap();

    assert_eq!(
        res,
        1.0 + 2.0 * (3.0f64.powf(4.0) - 5.0).powf(6.0 + 7.0 * 8.0) - 9.0
    );
}

#[test]
fn string_to_ast() {
    let parsed: AstStatement = "1 * (2 + 3)".parse().unwrap();

    let expected = expr![
        num!(1.0),
        op!(Multiply),
        op!(LeftParen),
        num!(2.0),
        op!(Add),
        num!(3.0),
        op!(RightParen)
    ];

    assert_eq!(parsed, expected);
}

#[test]
fn string_to_ast_hard() {
    let parsed: AstStatement = "1 + 2 * ( 3 ** 4 - 5 ) ** ( 6 + 7 * 8 ) - 9"
        .parse()
        .unwrap();

    let expected = expr![
        num!(1.0),
        op!(Add),
        num!(2.0),
        op!(Multiply),
        op!(LeftParen),
        num!(3.0),
        op!(Exponent),
        num!(4.0),
        op!(Subtract),
        num!(5.0),
        op!(RightParen),
        op!(Exponent),
        op!(LeftParen),
        num!(6.0),
        op!(Add),
        num!(7.0),
        op!(Multiply),
        num!(8.0),
        op!(RightParen),
        op!(Subtract),
        num!(9.0)
    ];

    assert_eq!(parsed, expected);
}

#[test]
fn functions() {
    let eval = AstEvaluator::new();
    macro_rules! test_fn {
        ($name: ident($($a: expr),*), $str: literal, $expected: expr) => {
            let parsed: AstStatement = $str.parse().unwrap();
            let f = expr!(fun!($name($($a),*)));
            assert_eq!(parsed, f);
            let res = eval.eval(&parsed).unwrap();
            assert_eq!(res, $expected);
        };
    }

    use std::f64::consts;

    test_fn!(
        sin(expr!(num!(3.0), op!(Multiply), var!(pi))),
        "sin(3 * pi)",
        f64::sin(3.0 * consts::PI)
    );

    test_fn!(
        round(expr!(num!(7.0), op!(Divide), num!(3.0))),
        "round(7 / 3)", // TODO: fractions in input
        f64::round(7.0 / 3.0)
    );

    test_fn!(
        log_7(expr!(num!(7.0), op!(Exponent), num!(6.0))),
        "log_7(7 ** 6)",
        6.0
    );

    test_fn!(
        gcd(expr!(num!(3.0)), expr!(num!(6.0)), expr!(num!(54.0))),
        "gcd(3, 6, 54)",
        3.0
    );
}
