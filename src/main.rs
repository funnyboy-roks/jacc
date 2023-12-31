use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
};

use anyhow::Context;
use clap::Parser;
use cli::{Cli, ContentSource};

use crate::ast::*;

mod ast;
mod cli;
mod format;

use format::ToStringRadix;

#[cfg(test)]
mod test;

fn run_from_reader<R>(cli: &Cli, buf: R) -> anyhow::Result<()>
where
    R: BufRead,
{
    let mut eval = AstEvaluator::new();

    let mut s = String::new();
    if !cli.quiet {
        print!("> ");
        stdout().flush()?;
    }
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

        if !cli.quiet {
            match cli.content_source()? {
                ContentSource::File(_) => println!("{}", line),
                ContentSource::Arg(_) => println!("{}", line),
                ContentSource::Stdin => {}
            }
        }

        let statement = line.parse()?;

        s.clear();

        let result = match eval.eval(&statement) {
            Ok(r) => r,
            Err(e) => {
                println!("Error evaluating equation: {:?}", e);
                print!("> ");
                stdout().flush()?;
                continue;
            }
        };

        eval.variable_map.insert("_".into(), result);

        if !cli.quiet {
            print!("{} = ", statement);
        }

        match cli.output_format() {
            lexer::NumberKind::Dec => println!("{}", result),
            lexer::NumberKind::Hex => println!("{}", result.to_string_radix::<16>()),
            lexer::NumberKind::Bin => println!("{}", result.to_string_radix::<2>()),
        };

        if !cli.quiet {
            print!("> ");
        }

        stdout().flush()?;
    }
    if !cli.quiet {
        println!("EOF");
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
