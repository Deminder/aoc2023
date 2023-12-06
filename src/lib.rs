use std::{
    fs::File,
    io::{BufReader, Lines, StdinLock},
};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub enum PuzzleInput {
    FileLines(Lines<BufReader<File>>),
    StdinLines(Lines<StdinLock<'static>>),
    StringLines(Box<dyn Iterator<Item = String>>),
}

impl Iterator for PuzzleInput {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PuzzleInput::FileLines(lines) => lines.next().and_then(|l| l.ok()),
            PuzzleInput::StdinLines(lines) => lines.next().and_then(|l| l.ok()),
            PuzzleInput::StringLines(lines) => lines.next(),
        }
    }
}

impl Into<PuzzleInput> for &'static str {
    fn into(self) -> PuzzleInput {
        PuzzleInput::StringLines(Box::new(
            self.split('\n').into_iter().map(|ss| ss.to_string()),
        ))
    }
}
pub type PuzzleSolutionFn = fn(PuzzleInput, bool) -> String;

pub fn puzzle_by_day(day: usize) -> Option<PuzzleSolutionFn> {
    match day {
        1 => Some(day1::solution),
        2 => Some(day2::solution),
        3 => Some(day3::solution),
        4 => Some(day4::solution),
        5 => Some(day5::solution),
        6 => Some(day6::solution),
        _ => None,
    }
}
