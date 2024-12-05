use crate::utils::file::read_lines;

use super::Day;

pub struct Day5 {}

struct Rule {
    before: usize,
    after: usize,
}

impl Day for Day5 {
    fn part1(&self, input_path: String) -> String {
        let lines = read_lines(input_path);
        let mut rules: Vec<Rule> = Vec::new();
        let mut sequances: Vec<Vec<usize>> = Vec::new();

        lines.iter().for_each(|line| {
            if line.contains("|") {
                let parts: Vec<&str> = line.split("|").collect();
                rules.push(Rule {
                    before: parts[0].trim().parse().unwrap(),
                    after: parts[1].trim().parse().unwrap(),
                });
            } else if line.contains(',') {
                sequances.push(
                    line.split(",")
                        .map(|part| part.trim().parse().unwrap())
                        .collect(),
                );
            }
        });

        let mut sum = 0;
        sequances.iter().for_each(|sequance| {
            if rules.iter().all(|rule| {
                match (
                    sequance.iter().position(|&r| r == rule.before),
                    sequance.iter().position(|&r| r == rule.after),
                ) {
                    (Some(before), Some(after)) => before < after,
                    _ => true,
                }
            }) {
                // println!("{:?}", sequance);
                // println!("{}", sequance[sequance.len() / 2]);
                sum += sequance[sequance.len() / 2];
            }
        });

        sum.to_string()
    }

    fn part2(&self, input_path: String) -> String {
        todo!()
    }
}

generate_day_tests!(5, "143", "123", "3608", "");
