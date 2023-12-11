use std::{fs, rc::Rc};

use anyhow::{anyhow, Context};

use puzzle::{puzzle_by_day, PuzzleInput, PuzzleSolutionFn};

#[derive(Debug, Clone)]
pub struct StringLines {
    content: Rc<String>,
    read_offset: usize,
}

impl StringLines {
    pub fn puzzle_input(self) -> PuzzleInput {
        PuzzleInput::StringLines(Box::new(self))
    }
}

impl From<String> for StringLines {
    fn from(value: String) -> Self {
        Self {
            content: Rc::new(value),
            read_offset: 0,
        }
    }
}

impl Iterator for StringLines {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.read_offset == self.content.len() {
            None
        } else {
            let line_len = self.content[self.read_offset..]
                .find('\n')
                .unwrap_or(self.content.len() - self.read_offset);
            let line = &self.content[self.read_offset..self.read_offset + line_len];
            self.read_offset += line_len + 1;
            Some(line.to_string())
        }
    }
}

pub fn bench_day(day: usize) -> anyhow::Result<(PuzzleSolutionFn, StringLines)> {
    let file_path = format!("src/day{day}/input.txt");

    let input: StringLines = fs::read_to_string(&file_path)
        .with_context(|| format!("Puzzle input '{file_path}' not found!"))?
        .into();
    let solution = puzzle_by_day(day).ok_or(anyhow!("Puzzle solution {day} not found!"))?;

    Ok((solution, input))
}
