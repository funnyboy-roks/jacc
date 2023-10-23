use std::fmt::Display;

use super::AstStatement;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    /// +
    Add,
    /// -
    Subtract,
    /// *
    Multiply,
    /// /
    Divide,
    /// %
    Modulo,
    // /// ~
    // Tilda,
    // /// !
    // Bang,
    /// ^
    Xor,
    /// &
    BitAnd,
    /// |
    BitOr,
    /// **
    Exponent,

    // /// =
    // Assign,
    /// (
    LeftParen,
    /// )
    RightParen,
    // /// {
    // LeftCurlyBracket,
    // /// }
    // RightCurlyBracket,
    // /// [
    // LeftSquareBracket,
    // /// ]
    // RightSquareBrace,
}

impl Operator {
    pub fn prec(&self) -> i64 {
        match self {
            Self::Add => 4,
            Self::Subtract => 4,

            Self::Multiply => 5,
            Self::Divide => 5,

            Self::Modulo => 5,
            //Self::Tilda => 1,
            //Self::Bang => 1,
            Self::Xor => 2,
            Self::BitAnd => 3,
            Self::BitOr => 1,

            Self::Exponent => 6,

            //Self::Assign => 1,
            Self::LeftParen => 0,
            Self::RightParen => 0,
            //Self::LeftCurlyBracket => 0,
            //Self::RightCurlyBracket => 0,
            //Self::LeftSquareBracket => 0,
            //Self::RightSquareBrace => 0,
        }
    }

    pub fn eval(&self, a: f64, b: f64) -> f64 {
        match self {
            Self::Add => a + b,
            Self::Subtract => a - b,
            Self::Multiply => a * b,
            Self::Divide => a / b,
            Self::Modulo => a % b,
            Self::Xor => (a as i64 ^ b as i64) as f64,
            Self::BitAnd => (a as i64 & b as i64) as f64,
            Self::BitOr => (a as i64 | b as i64) as f64,
            Self::Exponent => a.powf(b),

            Self::LeftParen => unreachable!(),
            Self::RightParen => unreachable!(),
            //Self::LeftCurlyBracket => unreachable!(),
            //Self::RightCurlyBracket => unreachable!(),
            //Self::LeftSquareBracket => unreachable!(),
            //Self::RightSquareBrace => unreachable!(),
        }
    }
}

impl From<Operator> for AstStatement {
    fn from(op: Operator) -> AstStatement {
        AstStatement::Operator(op)
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Add => "+",
                Self::Subtract => "-",
                Self::Multiply => "*",
                Self::Divide => "/",
                Self::Modulo => "%",
                //Self::Tilda => "~",
                //Self::Bang => "!",
                Self::Xor => "^",
                Self::BitAnd => "&",
                Self::BitOr => "|",
                Self::Exponent => "**",
                //Self::Assign => "=",
                Self::LeftParen => "(",
                Self::RightParen => ")",
                //Self::LeftCurlyBracket => "{",
                //Self::RightCurlyBracket => "}",
                //Self::LeftSquareBracket => "[",
                //Self::RightSquareBrace => "]",
            }
        )
    }
}
