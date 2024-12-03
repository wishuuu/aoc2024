use crate::utils::file::read_file;

use super::Day;

pub struct Day3 {}

impl Day for Day3 {
    fn part1(&self, input_path: String) -> String {
        let data = read_file(input_path);

        let regex = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        regex
            .captures_iter(&data)
            .fold(0, |acc, cap| {
                let a: i32 = cap[1].parse().unwrap();
                let b: i32 = cap[2].parse().unwrap();
                acc + a * b
            })
            .to_string()
    }

    fn part2(&self, input_path: String) -> String {
        let data = read_file(input_path);

        let regex = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
        let mut flag = true;

        regex
            .captures_iter(&data)
            .fold(0, |acc, cap| match (cap[0].as_ref(), flag) {
                ("do()", _) => {
                    flag = true;
                    acc
                }
                ("don't()", _) => {
                    flag = false;
                    acc
                }
                (_, true) => {
                    let a: i32 = cap[1].parse().unwrap();
                    let b: i32 = cap[2].parse().unwrap();
                    acc + a * b
                }
                _ => acc,
            })
            .to_string()
    }
}

generate_day_tests!(3, "161", "48", "185797128", "89798695");
