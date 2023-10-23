use std::{collections::HashMap, fmt::Display, str::FromStr};

use anyhow::{bail, ensure, Context};

pub mod functions;
pub mod lexer;
pub mod op;

use lexer::{Lexer, Token, TokenKind};
use op::Operator;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq)]
pub enum AstStatement {
    /// A number
    Number(f64),
    /// A variable reference
    Variable(String),
    /// A maths expression using Infix Notation (a + b) -- for evaluation, this gets convert to
    /// postfix (a b +)
    /// Technically `InfixExpression(vec![InfixExpression])` is invalid.
    InfixExpression(Vec<AstStatement>),
    Operator(Operator),
    /// A call to a built-in function
    /// All params should be evaluated before the function is called
    FunctionCall {
        name: String,
        params: Vec<AstStatement>,
    },
}

impl Display for AstStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstStatement::Number(n) => write!(f, "{}", n),
            AstStatement::Variable(v) => write!(f, "{}", v),
            AstStatement::InfixExpression(expr) => {
                for (i, e) in expr.iter().enumerate() {
                    write!(f, "{}", e)?;
                    if i != expr.len() - 1 {
                        write!(f, " ")?;
                    }
                }
                Ok(())
            }
            AstStatement::Operator(o) => write!(f, "{}", o),
            AstStatement::FunctionCall { name, params } => {
                write!(f, "{}(", name)?;
                for (i, p) in params.iter().enumerate() {
                    write!(f, "{}", p)?;
                    if i != params.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

impl AstStatement {
    pub fn is_operand(&self) -> bool {
        match self {
            AstStatement::Number(_) => true,
            AstStatement::Variable(_) => true,
            AstStatement::InfixExpression(_) => false,
            AstStatement::Operator(_) => false,
            AstStatement::FunctionCall { .. } => true,
        }
    }

    pub fn unwrap_operator(&self) -> Operator {
        match self {
            AstStatement::Operator(op) => *op,
            _ => unreachable!(),
        }
    }

    fn consume_function(
        ident: String,
        tokens: &[Token],
        i: &mut usize,
    ) -> anyhow::Result<AstStatement> {
        let mut args = Vec::new(); // All args for this function
        let mut curr_arg = Vec::new(); // the tokens in the current arg
        let mut depth = 0; // Paren depth -- 0 for same level as function
        while let Some(tok) = tokens.get(*i) {
            match tok.kind {
                TokenKind::RightParen if depth == 0 => {
                    args.push(Self::infix_expr_from_tokens(&curr_arg)?);
                    return Ok(AstStatement::FunctionCall {
                        name: ident,
                        params: args,
                    });
                }
                TokenKind::RightParen => {
                    depth -= 1;
                    curr_arg.push(tok.clone());
                }
                TokenKind::LeftParen => {
                    depth += 1;
                    curr_arg.push(tok.clone());
                }
                TokenKind::Comma => {
                    args.push(Self::infix_expr_from_tokens(&curr_arg)?);
                    curr_arg.clear();
                }
                TokenKind::Eof => bail!("Expected RightParen, found EOF"),

                _ => curr_arg.push(tok.clone()),
            }
            *i += 1;
        }
        todo!()
    }

    fn infix_expr_from_tokens(tokens: &Vec<Token>) -> anyhow::Result<AstStatement> {
        let mut stmts = Vec::new();
        let mut i = 0;
        while i < tokens.len() {
            let tok = &tokens[i];
            stmts.push(match &tok.kind {
                TokenKind::Number(n, _) => AstStatement::Number(*n),
                TokenKind::Ident(ident) => match tokens.get(i + 1).map(|t| &t.kind) {
                    Some(TokenKind::LeftParen) => {
                        i += 2; // skip the ident and the paren
                        Self::consume_function(ident.into(), tokens, &mut i)
                            .with_context(|| format!("parsing function: {}", ident))?
                    }
                    _ => AstStatement::Variable(ident.into()),
                },

                TokenKind::Plus => Operator::Add.into(),
                TokenKind::Minus => Operator::Subtract.into(),
                TokenKind::Asterisk => Operator::Multiply.into(),
                TokenKind::Slash => Operator::Divide.into(),
                TokenKind::Percent => Operator::Modulo.into(),

                TokenKind::Exponent => Operator::Exponent.into(),
                TokenKind::Carrot => Operator::Xor.into(),
                TokenKind::Ampersand => Operator::BitAnd.into(),
                TokenKind::Pipe => Operator::BitOr.into(),

                TokenKind::Comma => bail!("Unexpected token: ','"),

                TokenKind::LeftParen => Operator::LeftParen.into(),
                TokenKind::RightParen => Operator::RightParen.into(),

                TokenKind::LeftCurlyBracket => bail!("Unexpected token: '{{'"),
                TokenKind::RightCurlyBracket => bail!("Unexpected token: '}}'"),
                TokenKind::LeftSquareBracket => bail!("Unexpected token: '['"),
                TokenKind::RightSquareBrace => bail!("Unexpected token: ']'"),

                TokenKind::Newline => continue,

                TokenKind::Let => unimplemented!(),

                TokenKind::Eof => break, //bail!("Expected token, found EOF"),
                TokenKind::Invalid => bail!("Found invalid token"),
            });
            i += 1;
        }
        Ok(AstStatement::InfixExpression(stmts))
    }
}

impl FromStr for AstStatement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lexer = Lexer::new(s);
        Self::infix_expr_from_tokens(&lexer.collect())
    }
}

impl From<f64> for AstStatement {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

pub struct AstEvaluator {
    pub variable_map: HashMap<String, f64>,
    pub const_map: HashMap<&'static str, f64>,
    pub known_functions: functions::FnMap,
}

impl AstEvaluator {
    pub fn new() -> Self {
        let mut const_map = HashMap::new();
        const_map.insert("pi", std::f64::consts::PI);
        const_map.insert("e", std::f64::consts::E);
        const_map.insert("inf", f64::INFINITY);
        const_map.insert("true", 1.0);
        const_map.insert("false", 0.0);

        Self {
            variable_map: Default::default(),
            const_map,
            known_functions: functions::default_functions(),
        }
    }

    /// Evaluate a full [`AstStatement`] into its true value.  This does lookups into the
    /// constants and variables, runs functions, and evaluates operators
    pub fn eval(&self, statement: &AstStatement) -> anyhow::Result<f64> {
        Ok(match statement {
            AstStatement::Number(f) => *f,
            AstStatement::Variable(ref v) => self.get_variable(v.clone())?,
            AstStatement::InfixExpression(ref x) => self
                .eval_infix(x)
                .with_context(|| format!("Evaluating expression: '{}'", statement))?,
            AstStatement::FunctionCall {
                ref name,
                ref params,
            } => self
                .eval_function(name, params.to_vec())
                .with_context(|| format!("Evaluating function: '{}'", statement))?,
            AstStatement::Operator(o) => bail!("Expected expression, found {:?}", o),
        })
    }

    /// Get the value of a constant or variable (in that order)
    pub fn get_variable(&self, v: String) -> anyhow::Result<f64> {
        Ok(*self
            .const_map
            .get(v.as_str())
            .or_else(|| self.variable_map.get(&v))
            .with_context(|| format!("Undeclared variable or constant: '{}'", v))?)
    }

    /// Evaluate an infix expression by converting it to postfix and evaluating it
    fn eval_infix(&self, expr: &Vec<AstStatement>) -> anyhow::Result<f64> {
        // Convert the infix expression to postfix
        let pf = AstEvaluator::infix_to_postfix(expr)
            .context("Converting infix expression to postfix")?;

        // Evaluate the postfix

        // The stack will probably not actually grow to the length, but this is the max size that's
        // needed for it (I believe), so we'll set it to that to prevent allocations
        let mut s = Vec::with_capacity(pf.len());

        for tok in pf {
            if tok.is_operand() {
                s.push(tok);
            } else {
                // Pop the items from the stack (order matters)
                let b = s.pop().context("Missing items from postfix eval stack")?;
                let a = s.pop().context("Missing items from postfix eval stack")?;

                // Evaluate the statements, so that we can have recursive evals
                let a = self
                    .eval(&a)
                    .with_context(|| format!("Evaluating postfix expression: '{}'", a))?;
                let b = self
                    .eval(&b)
                    .with_context(|| format!("Evaluating postfix expression: '{}'", a))?;

                // Push the new computed value back onto the stack
                s.push(tok.unwrap_operator().eval(a, b).into());
            }
        }

        // More than one item left on stack means that we did something wrong
        ensure!(s.len() == 1, "Invalid expression");

        // Evaluate the final statement on the stack, this will be the answer
        self.eval(&s.pop().expect("checked with above ensure"))
    }

    /// Convert a infix expression into postfix notation in order to evaluate it more easily
    pub fn infix_to_postfix(expr: &Vec<AstStatement>) -> anyhow::Result<Vec<AstStatement>> {
        let mut out = Vec::new();
        let mut ops = Vec::new();

        for tok in expr {
            let tok = tok.clone();
            match tok {
                AstStatement::Operator(Operator::RightParen) => {
                    while ops.last() != Some(&Operator::LeftParen) {
                        out.push(ops.pop().context("Expected operator, found EOF")?.into());
                    }
                    ops.pop();
                }
                AstStatement::Operator(Operator::LeftParen) => {
                    ops.push(Operator::LeftParen);
                }
                AstStatement::Operator(tok) => {
                    while !ops.is_empty() && tok.prec() <= ops.last().unwrap().prec() {
                        let o = ops.pop().unwrap();
                        out.push(o.into());
                    }
                    ops.push(tok)
                }
                _ => {
                    out.push(tok);
                }
            }
        }

        // return the postfix queue and add on the remaining operators
        Ok(out
            .into_iter()
            .chain(ops.into_iter().map(Into::into).rev())
            .collect())
    }

    /// Evaluate a function based on its name.
    ///
    /// There are a few special cases:
    /// - `log_<BASE>(x)` will evaluate to the log using `<BASE>` as its base.
    ///     - i.e. `log_2(x)` will be log base 2
    /// - `log_B(x, base)` will evaluate to the log using `base` as its base.
    pub fn eval_function(&self, name: &str, args: Vec<AstStatement>) -> anyhow::Result<f64> {
        if let Some(s) = self.known_functions.get(name) {
            return s(self, args).with_context(|| format!("When evaluating function '{}'", name));
        }

        // Handle log_base functions
        if name.starts_with("log_") {
            if let Some((_, base)) = name.split_once('_') {
                // If the base is a number
                if let Ok(base) = base.parse::<f64>() {
                    ensure!(
                        // Ensure arg length requirements
                        args.len() == 1,
                        "Expected 1 arg, found {}. Usage: `log_{}(x)`",
                        args.len(),
                        base
                    );

                    return Ok(self.eval(&args[0])?.log(base));
                }

                // If the base is "B" use a custom base -- this could be done in the
                // `known_functions` but meh
                if base == "B" {
                    ensure!(
                        // Ensure arg length requirements
                        args.len() == 2,
                        "Expected 2 args, found {}. Usage: `log_B(x, base)`",
                        args.len()
                    );

                    return Ok(self.eval(&args[0])?.log(self.eval(&args[1])?));
                }

                bail!(
                    "Invalid log base: {}, specify the base in base10, for example: `log_2(x)`",
                    base
                );
            }
        }

        bail!("Unknown function: '{}'", name)
    }
}
