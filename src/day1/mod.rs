use regex::Regex;

use crate::PuzzleInput;

fn label_to_num(num_label: &str) -> u32 {
    if let Ok(num) = num_label.parse() {
        num
    } else {
        [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .into_iter()
        .enumerate()
        .find(|(_, label)| *label == num_label)
        .map(|(i, _)| i as u32 + 1)
        .unwrap()
    }
}

fn line_to_lables<'a>(line: &'a str, label_regex: &Regex) -> Vec<&'a str> {
    let mut start = 0;
    let mut labels = vec![];
    while let Some(m) = label_regex.find_at(line, start) {
        start = m.start() + 1;
        labels.push(m.as_str());
    }
    labels
}

fn run(input: PuzzleInput, part2: bool) -> u32 {
    let digit_regex = Regex::new(if part2 {
        r"\d|one|two|three|four|five|six|seven|eight|nine"
    } else {
        r"\d"
    })
    .unwrap();
    input
        .map(|line| {
            let labels = line_to_lables(&line, &digit_regex);
            let (first, last) = if let Some(f) = labels.first() {
                let first_num = label_to_num(f);
                (first_num, labels.last().map_or(first_num, |l| label_to_num(l)))
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
        assert_eq!(run(r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet".into(), false), 142);
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
7pqrstsixteen".into(),
                true
            ),
            281
        );
    }
}
