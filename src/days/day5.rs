use crate::utils::file::read_lines;

use super::Day;

pub struct Day5 {}

#[derive(Debug)]
struct Rule {
    before: usize,
    after: usize,
}

impl Day5 {
    fn read_data(&self, file_path: &str) -> (Vec<Rule>, Vec<Vec<usize>>) {
        let lines = read_lines(file_path.to_owned());
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
        (rules, sequances)
    }
}

impl Day for Day5 {
    fn part1(&self, input_path: String) -> String {
        let (rules, sequances) = self.read_data(&input_path);

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
        let (rules, sequances) = self.read_data(&input_path);
        let mut sum = 0;

        sequances
            .iter()
            .filter(|sequance| {
                rules.iter().any(|rule| {
                    match (
                        sequance.iter().position(|&r| r == rule.before),
                        sequance.iter().position(|&r| r == rule.after),
                    ) {
                        (Some(before), Some(after)) => before > after,
                        _ => false,
                    }
                })
            })
            .for_each(|sequance| {
                let mut seq = sequance.clone();
                for i in 0..seq.len() {
                    for j in 0..(seq.len() - i - 1) {
                        rules.iter().for_each(|rule| {
                            if seq[j] == rule.after && seq[j + 1] == rule.before {
                                seq.swap(j, j + 1);
                            }
                        });
                    }
                }
                sum += seq[seq.len() / 2];
            });
        sum.to_string()
    }
}

generate_day_tests!(5, "143", "123", "3608", "4922");
