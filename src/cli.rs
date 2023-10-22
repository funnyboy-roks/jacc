use std::path::PathBuf;

use anyhow::{bail, ensure};
use clap::Parser;

use crate::ast::lexer::NumberKind;

/// `maths` is a tool to do simple mathematics from the commandline without needing to know
/// anything fancy
///
/// One can run in three ways:
///
/// - using an argument: `maths '1 + 2 * 3'`
///
/// - by passing a file: `maths -f my_maths.txt`
///
/// - or by using the stdin: `echo '1 + 2 * 3' | maths`
#[derive(Clone, Debug, Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
    /// Ouput just the result and nothing else
    #[arg(short, long)]
    pub quiet: bool,

    /// Output the result in hex
    /// (conflicts with --bin)
    ///
    /// Fractions are displayed in hex with a `.`: 12.1875 (dec) => c.3 (hex)
    #[arg(short = 'x', long, conflicts_with = "bin")]
    pub hex: bool,

    /// Output the result in binary
    /// (conflicts with --hex)
    ///
    /// Fractions are displayed in binary with a `.`: 5.25 (dec) => 101.01 (bin)
    #[arg(short = 'b', long, conflicts_with = "hex")]
    pub bin: bool,

    /// The file from which to read maths input
    ///
    /// Each line in the file will be parsed as a separate expression, unless it ends with a `\`,
    /// in which case it will concatenate with the following line.
    #[arg(short, long, value_name = "file")]
    pub file: Option<PathBuf>,

    #[arg(conflicts_with = "file")]
    pub content: Option<String>,
}

impl Cli {
    pub fn content_source<'a>(&'a self) -> anyhow::Result<ContentSource<'a>> {
        Ok(match (&self.file, &self.content) {
            (None, None) => ContentSource::Stdin,
            (None, Some(content)) => ContentSource::Arg(&content),
            (Some(file), None) => ContentSource::File(&file),
            (Some(_), Some(_)) => unreachable!("clap handles conflict"),
        })
    }

    pub fn output_format(&self) -> NumberKind {
        match (self.hex, self.bin) {
            (true, true) => unreachable!("clap handles conflict"),
            (true, false) => NumberKind::Hex,
            (false, true) => NumberKind::Bin,
            (false, false) => NumberKind::Dec,
        }
    }
}

#[derive(Copy, Clone)]
pub enum ContentSource<'a> {
    File(&'a PathBuf),
    Arg(&'a str),
    Stdin,
}
