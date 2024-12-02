use itertools::Itertools;

use crate::utils::file::read_file;

use super::Day;

#[derive(Debug, Clone, Copy)]
pub struct Day2 {}

impl Day2 {
    fn get_data(&self, path: String) -> Vec<Vec<i32>> {
        let data = read_file(path);

        data.into_iter()
            .map(|s| {
                s.split(" ")
                    .map(|s| s.parse::<i32>().unwrap_or(0))
                    .collect::<Vec<i32>>()
            })
            .filter(|s| s.len() == 6)
            .collect()
    }
}

fn decreasing(data: &[i32]) -> bool {
    data.iter().tuple_windows().all(|(a, b)| a > b)
}
fn increasing(data: &[i32]) -> bool {
    data.iter().tuple_windows().all(|(a, b)| a < b)
}

impl Day for Day2 {
    fn part1(&self, input_path: String) -> String {
        let data = self.get_data(input_path);
        data.into_iter()
            .filter(|r| r.iter().tuple_windows().all(|(a, b)| (a - b).abs() < 4))
            .filter(|r| decreasing(r) || increasing(r))
            .count()
            .to_string()
    }

    fn part2(&self, input_path: String) -> String {
        "".into()
    }
}

generate_day_tests!(2, "2", "");
