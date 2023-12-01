use clap::Parser;
use puzzle::{puzzle_by_day, PuzzleInput};

use anyhow::Result;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Day of puzzle
    #[arg(value_name = "DAY")]
    day: usize,

    /// Part of puzzle to run
    #[arg(value_name = "PART", default_value_t = 1, value_parser = clap::value_parser!(u16).range(1..=2))]
    part: u16,

    /// Puzzle input file, otherwise reads from stdin
    #[arg(short, long, value_name = "FILE")]
    input_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let puzzle = puzzle_by_day(args.day).unwrap_or_else(|| {
        eprintln!("Puzzle day {} not found!", args.day);
        exit(1);
    });
    let lines = if let Some(path) = args.input_file {
        let file = File::open(path).unwrap_or_else(|err| {
            eprintln!("Failed opening puzzle input! {}", err);
            exit(1);
        });
        PuzzleInput::FileLines(io::BufReader::new(file).lines())
    } else {
        PuzzleInput::StdinLines(io::stdin().lines())
    };
    let solution = puzzle(lines, args.part == 2);

    println!("Part{}: {}", args.part, solution);
    Ok(())
}
