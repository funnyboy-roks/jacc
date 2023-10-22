use std::io::BufRead;

use ast::lexer::Lexer;

use crate::ast::*;

mod ast;

#[macro_use]
mod test;

fn main() -> anyhow::Result<()> {
    let eval = AstEvaluator::new();

    let stdin = std::io::stdin().lock();

    let mut s = String::new();
    for line in stdin.lines() {
        let line = line.unwrap();

        if line.ends_with('\\') {
            s.push_str(&line[..line.len() - 1]);
            continue;
        }

        let line = if s.is_empty() {
            &line
        } else {
            s.push_str(&line);
            &s
        };

        let lex = Lexer::new(line);

        s.clear();

        let statement = infix_expr_from_tokens(&lex.into_iter().collect())?;

        println!("{} = {}", statement, eval.eval(&statement)?);
    }
    Ok(())
}
