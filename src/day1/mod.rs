use crate::PuzzleInput;

fn labelstart_to_num(label_start: &str, part2: bool) -> Option<u32> {
    [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .into_iter()
    .position(|label| part2 && label_start.starts_with(label))
    .map(|pos| pos as u32 + 1)
    .or_else(|| label_start.chars().next().and_then(|c| c.to_digit(10)))
}

fn run(input: PuzzleInput, part2: bool) -> u32 {
    input
        .map(|line| {
            let mut nums = line
                .char_indices()
                .filter_map(|(i, _)| labelstart_to_num(&line[i..], part2));
            let (first, last) = if let Some(first_num) = nums.next() {
                (first_num, nums.last().unwrap_or(first_num))
            } else {
                (0, 0)
            };
            first * 10 + last
        })
        .sum()
}

pub fn solution(input: PuzzleInput, part2: bool) -> String {
    run(input, part2).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(
            run(
                r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
                    .into(),
                false
            ),
            142
        );
        assert_eq!(run("soneighttwo39ktl132".into(), true), 12);
        assert_eq!(run("oneight".into(), true), 18);
        assert_eq!(run("oneightwo".into(), true), 12);
        assert_eq!(
            run(
                r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
                    .into(),
                true
            ),
            281
        );
    }
}
