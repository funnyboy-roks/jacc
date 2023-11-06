#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NumberKind {
    /// 123
    Dec,
    /// 0x123
    Hex,
    /// 0b101
    Bin,
}

impl NumberKind {
    /// Get the radix of the specific numberkind
    pub fn radix(&self) -> u32 {
        match self {
            Self::Dec => 10,
            Self::Hex => 16,
            Self::Bin => 2,
        }
    }

    /// Parse a character from the given numberkind's radix
    pub fn parse(&self, s: &str) -> Option<f64> {
        match self {
            Self::Dec => s.parse().ok(),
            _ => {
                if let Some((whole, frac)) = s.split_once('.') {
                    let radix = self.radix();
                    let num = u64::from_str_radix(whole, radix).ok()? as f64;
                    let mut frac = u64::from_str_radix(frac, radix).ok()? as f64;
                    while frac >= 1.0 {
                        frac /= radix as f64;
                    }
                    Some(num + frac)
                } else {
                    Some(u64::from_str_radix(s, self.radix()).ok()? as f64)
                }
            }
        }
    }

    /// Check if a char is a valid digit for this radix
    pub fn is_valid_digit(&self, c: char) -> bool {
        c == '.' || c.is_digit(self.radix())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// 123
    /// 0x123
    /// 0b00010101
    Number(f64, NumberKind),
    Ident(String),
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Asterisk,
    /// /
    Slash,
    /// %
    Percent,
    /// **
    Exponent,
    // /// ~
    // Tilda,
    // /// !
    // Bang,
    /// ^
    Carrot,
    /// &
    Ampersand,
    /// |
    Pipe,

    /// ,
    Comma,

    // /// =
    // Assign,
    /// (
    LeftParen,
    /// )
    RightParen,
    /// {
    LeftCurlyBracket,
    /// }
    RightCurlyBracket,
    /// [
    LeftSquareBracket,
    /// ]
    RightSquareBrace,

    /// \n
    Newline,

    /// let
    Let,

    /// End of file
    Eof,
    /// Invalid Token
    Invalid,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TextSpan {
    start: usize,
    end: usize,
}

impl TextSpan {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Print a text span highlighting the correct region in the string
    pub fn print(&self, s: &str) {
        const MIN_BEGIN: usize = 20;

        let chars = s.chars();
        let chars = chars.skip(std::cmp::max(MIN_BEGIN, self.start) - MIN_BEGIN);
        let chars = chars.take(self.end - self.start + MIN_BEGIN * 2);
        println!("{}", chars.collect::<String>());
        println!(
            "{}{}",
            " ".repeat(std::cmp::min(MIN_BEGIN, self.start)),
            "^".repeat(self.end - self.start)
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

/// Convert a character to a number by subtracting '0'
///
/// Note: `c` _must_ be valid ascii between '0'..='9' for a correct result
fn char_to_num(c: char) -> f64 {
    (c as u8 - b'0') as f64
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexer {
    input: Vec<char>,
    index: usize,
}

impl Lexer {
    /// Create a new lexer from a given input
    pub fn new<S>(input: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            input: input.as_ref().chars().collect(),
            index: 0,
        }
    }

    /// Check the value of the next character
    fn peek_char(&self) -> Option<char> {
        if self.index + 1 == self.input.len() {
            None
        } else {
            Some(self.input[self.index + 1])
        }
    }

    /// Consume one number from the input and return it as an f64 and NumberKind
    fn take_num(&mut self) -> Option<(f64, NumberKind)> {
        // Match the first digit
        let Some(c) = self.current_char() else {
            return None;
        };

        //eprintln!("lead digit: '{}'", c);
        let lead_digit = if c.is_ascii_digit() {
            c
        } else {
            //eprintln!("No lead digit");
            return None;
        };

        let mut kind = NumberKind::Dec;
        let mut number = String::new();

        // Match the second char (might not be a digit if hex or bin)
        if let Some(c) = self.peek_char() {
            if lead_digit == '0' {
                match c {
                    'x' => {
                        kind = NumberKind::Hex;
                    }
                    'b' => {
                        kind = NumberKind::Bin;
                    }
                    '.' | '0'..='9' => {
                        // If we want octal support, add it here:
                        // kind = NumberKind::Oct;
                        number.push(lead_digit);
                        number.push(c);
                    }
                    // End of number
                    _ => {
                        self.take_char();
                        return Some((0.0, kind));
                    }
                }
                self.take_char();
            } else {
                number.push(lead_digit);
                self.take_char();
                if kind.is_valid_digit(c) {
                    //eprintln!("'{}' is valid", c);
                    number.push(c);
                } else {
                    //eprintln!("returning {}", number);
                    return Some((kind.parse(&number)?, kind));
                }
            }
        } else {
            self.take_char();
            return Some((char_to_num(lead_digit), kind));
        }

        let mut digits = 1;
        while let Some(c) = self.peek_char() {
            //dbg!(c);
            if !kind.is_valid_digit(c) {
                break;
            }
            number.push(c);
            digits += 1;
            self.index += 1;
        }

        if digits == 0 {
            None
        } else {
            self.index += 1;
            Some((kind.parse(&number)?, kind))
        }
    }

    /// Consume one complete number
    fn consume_number(&mut self) -> Option<Token> {
        let start = self.index;
        let num = self.take_num();

        num.map(|(n, kind)| {
            Token::new(TokenKind::Number(n, kind), TextSpan::new(start, self.index))
        })
    }

    /// Get the character that the lexer is current at
    fn current_char(&self) -> Option<char> {
        self.input.get(self.index).copied()
    }

    /// Consume one complete ident from the input
    fn consume_ident(&mut self) -> Option<Token> {
        if let Some(c) = self.current_char() {
            let start = self.index;
            let mut name = String::new();
            match c {
                '_' | 'a'..='z' | 'A'..='Z' => {
                    self.take_char();
                    name.push(c);
                    while let Some(c) = self.current_char() {
                        match c {
                            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                                self.take_char();
                                name.push(c);
                            }
                            _ => break,
                        }
                    }
                }
                _ => return None,
            }

            //eprintln!("emitting ident: {}", name);
            Some(Token::new(
                match name.as_str() {
                    "let" => TokenKind::Let,
                    _ => TokenKind::Ident(name),
                },
                TextSpan::new(start, self.index),
            ))
        } else {
            None
        }
    }

    /// Take one character from the input.
    fn take_char(&mut self) -> Option<char> {
        if self.index < self.input.len() {
            self.index += 1;
            Some(self.input[self.index - 1])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_take_number() {
        let mut lex = Lexer::new("123");
        let (n, kind) = lex.take_num().unwrap();

        assert_eq!((n, kind), (123.0, NumberKind::Dec));

        let mut lex = Lexer::new("123.5123");
        let (n, kind) = lex.take_num().unwrap();

        assert_eq!((n, kind), (123.5123, NumberKind::Dec));

        let mut lex = Lexer::new("0x123");
        let (n, kind) = lex.take_num().unwrap();

        assert_eq!((n, kind), (0x123 as f64, NumberKind::Hex));

        let mut lex = Lexer::new("0b101");
        let (n, kind) = lex.take_num().unwrap();

        assert_eq!((n, kind), (0b101 as f64, NumberKind::Bin));
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.input.len() {
            self.index += 1;
            return Some(Token::new(
                TokenKind::Eof,
                TextSpan::new(self.index - 1, self.index),
            ));
        }

        if self.index > self.input.len() {
            return None;
        }

        let tok = loop {
            let prev_index = self.index;
            let (tok, kind) = match self.current_char() {
                Some('+') => (None, Some(TokenKind::Plus)),
                Some('-') => {
                    if matches!(self.peek_char(), Some('.' | '0'..='9')) {
                        self.take_char();
                        if let Some(mut tok) = self.consume_number() {
                            tok.kind = match tok.kind {
                                TokenKind::Number(n, t) => TokenKind::Number(-n, t),
                                _ => unreachable!(
                                    "Self::consume_number should always return a number"
                                ),
                            };
                            (Some(tok), None)
                        } else {
                            (None, Some(dbg!(TokenKind::Invalid)))
                        }
                    } else {
                        (None, Some(TokenKind::Minus))
                    }
                }
                Some('*') => {
                    if self.peek_char() == Some('*') {
                        self.take_char();
                        (None, Some(TokenKind::Exponent))
                    } else {
                        (None, Some(TokenKind::Asterisk))
                    }
                }
                Some('/') => (None, Some(TokenKind::Slash)),
                Some('%') => (None, Some(TokenKind::Percent)),
                //Some('~') => (None, Some(TokenKind::Tilda)),
                //Some('!') => (None, Some(TokenKind::Bang)),
                Some('^') => (None, Some(TokenKind::Carrot)),
                Some('&') => (None, Some(TokenKind::Ampersand)),
                Some('|') => (None, Some(TokenKind::Pipe)),

                Some(',') => (None, Some(TokenKind::Comma)),

                //Some('=') => (None, Some(TokenKind::Assign)),
                Some('(') => (None, Some(TokenKind::LeftParen)),
                Some(')') => (None, Some(TokenKind::RightParen)),
                Some('{') => (None, Some(TokenKind::LeftCurlyBracket)),
                Some('}') => (None, Some(TokenKind::RightCurlyBracket)),
                Some('[') => (None, Some(TokenKind::LeftSquareBracket)),
                Some(']') => (None, Some(TokenKind::RightSquareBrace)),

                Some('\n') => (None, Some(TokenKind::Newline)),

                Some(c) if c.is_whitespace() => {
                    self.take_char();
                    continue;
                }

                Some('0'..='9' | '.') => {
                    if let Some(tok) = self.consume_number() {
                        (Some(tok), None)
                    } else {
                        (None, Some(dbg!(TokenKind::Invalid)))
                    }
                }
                Some('_' | 'a'..='z' | 'A'..='Z') => {
                    if let Some(n) = self.consume_ident() {
                        (Some(n), None)
                    } else {
                        (None, Some(dbg!(TokenKind::Invalid)))
                    }
                }
                Some(_) => (None, Some(TokenKind::Invalid)),
                None => (None, Some(TokenKind::Eof)),
            };

            break (if let Some(tok) = tok {
                tok
            } else {
                self.index += 1;
                Token::new(kind.unwrap(), TextSpan::new(prev_index, self.index))
            });
        };

        Some(tok)
    }
}
