use std::collections::HashMap;

pub mod day1;

pub trait Day {
    fn part1(&self, input_path: String) -> String;
    fn part2(&self, intpu_path: String) -> String;
}

pub struct Runner {
    days: HashMap<u8, Box<dyn Day>>,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            days: [
                (1u8, day1::Day1 {}),
                // (2, Box::new(day1::Day2 {})),
                // (3, Box::new(day1::Day3 {})),
                // (4, Box::new(day1::Day4 {})),
                // (5, Box::new(day1::Day5 {})),
                // (6, Box::new(day1::Day6 {})),
                // (7, Box::new(day1::Day7 {})),
                // (8, Box::new(day1::Day8 {})),
                // (9, Box::new(day1::Day9 {})),
                // (10, Box::new(day1::Day10 {})),
                // (11, Box::new(day1::Day11 {})),
                // (12, Box::new(day1::Day12 {})),
                // (13, Box::new(day1::Day13 {})),
                // (14, Box::new(day1::Day14 {})),
                // (15, Box::new(day1::Day15 {})),
                // (16, Box::new(day1::Day16 {})),
                // (17, Box::new(day1::Day17 {})),
                // (18, Box::new(day1::Day18 {})),
                // (19, Box::new(day1::Day19 {})),
                // (20, Box::new(day1::Day20 {})),
                // (21, Box::new(day1::Day21 {})),
                // (22, Box::new(day1::Day22 {})),
                // (23, Box::new(day1::Day23 {})),
                // (24, Box::new(day1::Day24 {})),
                // (25, Box::new(day1::Day25 {})),
            ]
            .iter()
            .map(|(key, day)| (*key, Box::new(*day) as Box<dyn Day>))
            .collect(),
        }
    }
    pub fn run(&self) {
        for (day, _) in self.days.iter() {
            self.run_day(*day);
        }
    }
    pub fn run_day(&self, day: u8) {
        println!("Day: {}", day);
        self.run_day_part(day, 1);
        self.run_day_part(day, 2);
    }

    pub fn run_day_part(&self, day: u8, part: u8) {
        if let Some(day) = self.days.get(&day) {
            match part {
                1 => println!("Part 1: {}", day.part1("inputs/d1.task".into())),
                2 => println!("Part 2: {}", day.part2("inputs/d1.task".into())),
                _ => println!("Invalid part number"),
            }
        } else {
            println!("Day {} not found", day);
        }
    }
}
