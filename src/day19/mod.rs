use itertools::Itertools;
use regex::Regex;

use crate::{range_intersect, split_by_empty_line, PuzzleInput};

use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

type Num = usize;
type Category = usize;

const NUM_MAX: Num = 4000;
const NUM_MIN: Num = 1;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Constraint {
    variables: [Range<Num>; 4],
}

impl Constraint {
    fn empty() -> Self {
        Constraint {
            variables: array_init::array_init(|_| NUM_MIN..NUM_MAX + 1),
        }
    }
    fn accept(&self, props: &[Num; 4]) -> bool {
        props
            .iter()
            .zip_eq(self.variables.iter())
            .all(|(num, range)| range.contains(num))
    }

    fn tighten(mut self, cat: Category, conjunction: &Range<Num>) -> Option<Self> {
        let range = &mut self.variables[cat];
        if let Some(inter) = range_intersect(range, conjunction) {
            *range = inter;
            Some(self)
        } else {
            None
        }
    }

    fn count_combinations(&self) -> usize {
        self.variables.iter().map(|r| r.end - r.start).product()
    }
}

fn find_constrains(rules: &HashMap<String, Vec<RuleSegment>>) -> Vec<Constraint> {
    let mut constraints: HashSet<Constraint> = HashSet::new();
    let mut constraint_stack = vec![("in".to_string(), 0, Constraint::empty())];

    while let Some((rule_label, segment_index, constraint)) = constraint_stack.pop() {
        match &rules[&rule_label][segment_index] {
            RuleSegment::Literal(RuleLiteral::Accept) => {
                constraints.insert(constraint);
            }
            RuleSegment::Literal(RuleLiteral::Reject) => {}
            RuleSegment::Literal(RuleLiteral::Link(label)) => {
                constraint_stack.push((label.clone(), 0, constraint));
            }
            RuleSegment::Condition(cat, less_than, num, literal) => {
                let (conjunction, else_conjunction) = if *less_than {
                    (NUM_MIN..*num, *num..NUM_MAX + 1)
                } else {
                    (*num + 1..NUM_MAX + 1, NUM_MIN..*num + 1)
                };
                if let Some(else_constraint) = constraint.clone().tighten(*cat, &else_conjunction) {
                    constraint_stack.push((rule_label.clone(), segment_index + 1, else_constraint));
                }
                if let Some(if_constraint) = constraint.tighten(*cat, &conjunction) {
                    match literal {
                        RuleLiteral::Reject => {}
                        RuleLiteral::Accept => {
                            constraints.insert(if_constraint);
                        }
                        RuleLiteral::Link(label) => {
                            constraint_stack.push((label.clone(), 0, if_constraint));
                        }
                    }
                }
            }
        }
    }
    constraints.into_iter().collect()
}

fn count_combinations(constraints: &[Constraint]) -> usize {
    constraints.iter().map(|c| c.count_combinations()).sum()
}

#[derive(Debug)]
enum RuleLiteral {
    Reject,
    Accept,
    Link(String),
}

#[derive(Debug)]
enum RuleSegment {
    Literal(RuleLiteral),
    Condition(Category, bool, Num, RuleLiteral),
}

fn run(input: PuzzleInput, part2: bool) -> usize {
    let [rule_pattern, segment_pattern, num_pattern] =
        [r"(\w+)\{(.*?)\}", r"([xmas])(.)(\d+):(\w+)", r"\d+"].map(|r| Regex::new(r).unwrap());
    split_by_empty_line!(input)
        .scan(None as Option<Vec<Constraint>>, |constraints, lines| {
            if let Some(consts) = constraints.take() {
                let parts: Vec<[Num; 4]> = lines
                    .map(|l| {
                        array_init::from_iter(
                            num_pattern
                                .find_iter(&l)
                                .map(|c| c.as_str().parse().expect("part num")),
                        )
                        .unwrap()
                    })
                    .collect_vec();
                Some(Some(
                    parts
                        .into_iter()
                        .filter(|p| consts.iter().any(|c| c.accept(p)))
                        .map(|p| p.iter().sum::<usize>())
                        .sum(),
                ))
            } else {
                let rules: HashMap<String, Vec<RuleSegment>> = lines
                    .map(|line| {
                        let caps = rule_pattern.captures(&line).expect("rule line");
                        (
                            caps[1].to_string(),
                            caps[2]
                                .split(',')
                                .map(|p| {
                                    segment_pattern
                                        .captures(p)
                                        .map(|c| {
                                            RuleSegment::Condition(
                                                "xmas".find(&c[1]).unwrap(),
                                                &c[2] == "<",
                                                c[3].parse().expect("condition num"),
                                                match &c[4] {
                                                    "A" => RuleLiteral::Accept,
                                                    "R" => RuleLiteral::Reject,
                                                    l => RuleLiteral::Link(l.to_string()),
                                                },
                                            )
                                        })
                                        .unwrap_or_else(|| {
                                            RuleSegment::Literal(match p {
                                                "A" => RuleLiteral::Accept,
                                                "R" => RuleLiteral::Reject,
                                                _ => RuleLiteral::Link(p.to_string()),
                                            })
                                        })
                                })
                                .collect(),
                        )
                    })
                    .collect();
                let c = find_constrains(&rules);
                if part2 {
                    Some(Some(count_combinations(&c)))
                } else {
                    *constraints = Some(c);
                    Some(None)
                }
            }
        })
        .flatten()
        .next()
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
        let test_input = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}

";
        assert_eq!(run(test_input.into(), false), 19114);
        assert_eq!(run(test_input.into(), true), 167409079868000);
    }
}
