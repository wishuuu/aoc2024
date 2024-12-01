use std::collections::HashMap;

#[macro_use]
pub mod macros;

pub mod day1;
pub mod day2;

pub trait Day {
    fn part1(&self, input_path: String) -> String;
    fn part2(&self, input_path: String) -> String;
}

pub struct Runner {
    days: HashMap<u8, Box<dyn Day>>,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            days: [
                (1u8, Box::new(day1::Day1 {}) as Box<dyn Day>),
                (2u8, Box::new(day2::Day2 {}) as Box<dyn Day>),
            ]
            .into_iter()
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

    pub fn run_day_part(&self, d: u8, part: u8) {
        if let Some(day) = self.days.get(&d) {
            match part {
                1 => println!("Part 1: {}", day.part1(format!("inputs/d{}.task", d))),
                2 => println!("Part 2: {}", day.part2(format!("inputs/d{}.task", d))),
                _ => println!("Invalid part number"),
            }
        } else {
            println!("Day {} not found", d);
        }
    }
}
