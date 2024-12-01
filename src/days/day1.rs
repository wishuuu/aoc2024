use crate::structures::counter::Counter;

use super::Day;

#[derive(Debug, Clone, Copy)]
pub struct Day1 {}

impl Day1 {
    fn read_file(&self, path: String) -> Vec<String> {
        std::fs::read_to_string(path)
            .unwrap()
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    fn get_data(&self, path: String) -> (Vec<i32>, Vec<i32>) {
        let data = self.read_file(path);

        let (d1, d2): (Vec<_>, Vec<_>) = data
            .into_iter()
            .map(|s| {
                s.split("   ")
                    .map(|s| s.parse::<i32>().unwrap_or(0))
                    .collect::<Vec<i32>>()
            })
            .filter(|s| s.len() >= 2)
            .map(|v| (v[0], v[1]))
            .unzip();

        (d1, d2)
    }
}

impl Day for Day1 {
    fn part1(&self, path: String) -> String {
        let (mut d1, mut d2) = self.get_data(path);

        d1.sort();
        d2.sort();

        let mut sum = 0;
        for it in d1.iter().zip(d2.iter()) {
            sum += (it.0 - it.1).abs()
        }
        sum.to_string()
    }

    fn part2(&self, path: String) -> String {
        let (d1, d2) = self.get_data(path);

        let d2: Counter<i32> = d2.into_iter().collect();
        d1.into_iter()
            .fold(0, |acc, x| acc + x * d2.get(x) as i32)
            .to_string()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let day = Day1 {};
        assert_eq!(day.part1("inputs/d1.test".into()), "11");
    }
    #[test]
    fn test_part2() {
        let day = Day1 {};
        assert_eq!(day.part2("inputs/d1.test".into()), "31");
    }
}
