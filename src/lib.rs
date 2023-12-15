use std::{
    fs::File,
    io::{BufReader, Lines, StdinLock},
};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub enum PuzzleInput {
    FileLines(Lines<BufReader<File>>),
    StdinLines(Lines<StdinLock<'static>>),
    StringLines(Box<dyn Iterator<Item = String>>),
}

#[macro_export]
macro_rules! split_by_empty_line {
    ($input: expr) => {
        $input
            .group_by(|line| line.is_empty())
            .into_iter()
            .skip_while(|(empty, _)| *empty)
            .step_by(2)
            .map(|(_, lines)| lines)
    };
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

impl From<&'static str> for PuzzleInput {
    fn from(val: &'static str) -> Self {
        PuzzleInput::StringLines(Box::new(val.split('\n').map(|ss| ss.to_string())))
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
        7 => Some(day7::solution),
        8 => Some(day8::solution),
        9 => Some(day9::solution),
        10 => Some(day10::solution),
        11 => Some(day11::solution),
        12 => Some(day12::solution),
        13 => Some(day13::solution),
        14 => Some(day14::solution),
        15 => Some(day15::solution),
        _ => None,
    }
}
