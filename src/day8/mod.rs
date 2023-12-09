use itertools::Itertools;

use crate::PuzzleInput;
mod network;
use network::{Node, NodeNetwork};

pub fn steps_until_end(
    network: &NodeNetwork,
    start_node: Node,
    go_right_instructions: &[bool],
    part2: bool,
) -> usize {
    go_right_instructions
        .iter()
        .copied()
        .cycle()
        .scan(start_node, |pos: &mut Node, go_right| {
            let (left, right) = network[*pos];
            *pos = if go_right { right } else { left };
            Some(*pos)
        })
        .position(|pos| pos.is_end_node(part2))
        .unwrap()
        + 1
}

fn run(mut input: PuzzleInput, part2: bool) -> u64 {
    let go_right_instructions = input
        .next()
        .unwrap()
        .chars()
        .map(|c| c == 'R')
        .collect_vec();
    let network: NodeNetwork = input
        .filter(|line| !line.is_empty())
        .map(|line| [&line[0..3], &line[7..10], &line[12..15]].map(|v| v.into()))
        .collect();

    // Part2: Each ghost is starting at "??A" (??A = (ABC, DEF)
    // and reaches "??E" (??Z = (DEF, ABC)) after `n` steps.
    //
    // This is not mentioned in the puzzle description but is required for correctness:
    // When starting at an ending, no other endings can be reached
    // and it again takes `n` steps to cycle back to the same ending.
    network
        .start_nodes(part2)
        .map(|n| steps_until_end(&network, n, &go_right_instructions, part2) as u64)
        .reduce(num_integer::lcm)
        .unwrap()
}

pub fn solution(input: PuzzleInput, part2: bool) -> String {
    run(input, part2).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let test_input = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(run(test_input.into(), false), 2);
    }

    #[test]
    fn test_run2() {
        let test_input2 = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(run(test_input2.into(), false), 6);
    }

    #[test]
    fn test_run_for_part2() {
        let test_input3 = r"LR

NNA = (NNB, XXX)
NNB = (XXX, NNZ)
NNZ = (NNB, XXX)
MMA = (MMB, XXX)
MMB = (MMC, MMC)
MMC = (MMZ, MMZ)
MMZ = (MMB, MMB)
XXX = (XXX, XXX)";
        assert_eq!(run(test_input3.into(), true), 6);
    }
}
