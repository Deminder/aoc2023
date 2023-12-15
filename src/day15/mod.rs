use crate::PuzzleInput;

#[derive(Debug)]
enum Operation {
    Remove,
    Set(u8),
}

fn hash(s: &str) -> u8 {
    s.bytes()
        .fold(0, |h, code| h.wrapping_add(code).wrapping_mul(17))
}

fn run(line: &str, part2: bool) -> usize {
    if part2 {
        let mut boxes = [0; 256].map(|_| vec![]);
        for (label, op) in line.split(',').map(|s| {
            let op = s
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .map(|d| Operation::Set(d as u8))
                .unwrap_or(Operation::Remove);
            (
                &s[..s.len()
                    - match op {
                        Operation::Remove => 1,
                        Operation::Set(_) => 2,
                    }],
                op,
            )
        }) {
            let list = &mut boxes[hash(label) as usize];
            if let Some(index) = list.iter().position(|(k, _)| *k == label) {
                match op {
                    Operation::Set(b) => {
                        list[index].1 = b;
                    }
                    Operation::Remove => {
                        list.remove(index);
                    }
                };
            } else if let Operation::Set(b) = op {
                list.push((label, b));
            }
        }
        boxes
            .into_iter()
            .enumerate()
            .map(|(box_index, list)| {
                (box_index + 1)
                    * list
                        .into_iter()
                        .enumerate()
                        .map(|(slot_index, (_, focal_length))| {
                            (slot_index + 1) * (focal_length as usize)
                        })
                        .sum::<usize>()
            })
            .sum()
    } else {
        line.split(',').map(|s| hash(s) as usize).sum()
    }
}

pub fn solution(mut input: PuzzleInput, part2: bool) -> String {
    run(&input.next().unwrap(), part2).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let test_input = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(run(test_input, false), 1320);
        assert_eq!(run(test_input, true), 145);
    }
}
