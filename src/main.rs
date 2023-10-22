use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Context;
use ast::lexer::Lexer;
use clap::Parser;
use cli::{Cli, ContentSource};

use crate::ast::*;

mod ast;
mod cli;
mod format;

use format::{ToBin, ToHex};

#[cfg(test)]
mod test;

fn run_from_reader<R>(cli: &Cli, buf: R) -> anyhow::Result<()>
where
    R: BufRead,
{
    let eval = AstEvaluator::new();

    let mut s = String::new();
    for line in buf.lines() {
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

        let statement = line.parse()?;

        s.clear();

        if !cli.quiet {
            print!("{} = ", statement);
        }

        let result = eval.eval(&statement)?;

        match cli.output_format() {
            lexer::NumberKind::Dec => println!("{}", result),
            lexer::NumberKind::Hex => println!("{}", result.to_hex()),
            lexer::NumberKind::Bin => println!("{}", result.to_bin()),
        };
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.content_source()? {
        ContentSource::File(f) => {
            let file = File::open(f).context("Opening file for reading")?;
            let buf = BufReader::new(file);
            run_from_reader(&cli, buf)?;
        }
        ContentSource::Arg(a) => {
            run_from_reader(&cli, a.as_bytes())?;
        }
        ContentSource::Stdin => {
            let lock = std::io::stdin().lock();
            run_from_reader(&cli, lock)?;
        }
    }

    Ok(())
}
