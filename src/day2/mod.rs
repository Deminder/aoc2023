use itertools::Itertools;
use regex::Regex;

use crate::PuzzleInput;

fn run(input: &str, part2: bool) -> u32 {
    0
}

pub fn solution(mut input: PuzzleInput, part2: bool) -> String {
    run(&input.join("\n"), part2).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run(r"", false), 0);
        assert_eq!(run(r"", true), 0);
    }
}
