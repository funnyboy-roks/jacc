use super::*;

#[test]
fn infix_to_postfix() {
    let expr: Vec<_> = vec![
        AstStatement::Variable("a".into()),
        AstStatement::Operator(Operator::Add),
        AstStatement::Variable("b".into()),
        AstStatement::Operator(Operator::Multiply),
        AstStatement::Variable("c".into()),
    ]
    .into_iter()
    .map(Box::new)
    .collect();

    let out = AstEvaluator::infix_to_postfix(&expr);
    assert_eq!(
        out.unwrap(),
        vec![
            AstStatement::Variable("a".into()),
            AstStatement::Variable("b".into()),
            AstStatement::Variable("c".into()),
            AstStatement::Operator(Operator::Multiply),
            AstStatement::Operator(Operator::Add),
        ]
    );
}

#[test]
fn eval_infix() {
    let expr: Vec<_> = vec![
        AstStatement::Variable("a".into()),
        AstStatement::Operator(Operator::Add),
        AstStatement::Variable("b".into()),
        AstStatement::Operator(Operator::Multiply),
        AstStatement::Variable("c".into()),
    ]
    .into_iter()
    .map(Box::new)
    .collect();

    let mut vars = HashMap::new();
    let a = 2.0;
    let b = 3.0;
    let c = 4.0;
    vars.insert("a".into(), a);
    vars.insert("b".into(), b);
    vars.insert("c".into(), c);

    let eval = AstEvaluator {
        variable_map: vars,
        known_functions: Default::default(),
        const_map: Default::default(),
    };

    assert_eq!(
        eval.eval(&AstStatement::InfixExpression(expr)).unwrap(),
        a + b * c
    );
}

#[test]
pub fn test_tokeniser() {
    use super::lexer::{NumberKind::*, TextSpan, Token, TokenKind::*};

    let s = "1 12 0 0x3 let x **";
    let lex = Lexer::new(s);

    macro_rules! span {
        ($a:literal..=$b:literal) => {
            TextSpan::new($a, $b)
        };
    }

    let expected = vec![
        Token {
            kind: Number(1.0, Dec),
            span: span!(0..=1),
        },
        Token {
            kind: Number(12.0, Dec),
            span: span!(2..=4),
        },
        Token {
            kind: Number(0.0, Dec),
            span: span!(5..=6),
        },
        Token {
            kind: Number(3.0, Hex),
            span: span!(7..=10),
        },
        Token {
            kind: Let,
            span: span!(11..=14),
        },
        Token {
            kind: Ident("x".to_string()),
            span: span!(15..=16),
        },
        Token {
            kind: Exponent,
            span: span!(17..=19),
        },
        Token {
            kind: Eof,
            span: span!(19..=20),
        },
    ];

    assert_eq!(lex.collect::<Vec<_>>(), expected);
}
