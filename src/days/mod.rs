use std::collections::HashMap;

use crate::Args;

#[macro_use]
pub mod macros;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub trait Day {
    fn part1(&self, input_path: String) -> String;
    fn part2(&self, input_path: String) -> String;
}

pub enum RunnerOption {
    Task(Option<u8>),
    Test(Option<u8>),
}

pub struct Runner {
    days: HashMap<u8, Box<dyn Day>>,
    runner_option: RunnerOption,
}

impl Runner {
    pub fn new(args: Args) -> Self {
        Self {
            days: [
                (1u8, Box::new(day1::Day1 {}) as Box<dyn Day>),
                (2u8, Box::new(day2::Day2 {}) as Box<dyn Day>),
                (3u8, Box::new(day3::Day3 {}) as Box<dyn Day>),
                (4u8, Box::new(day4::Day4 {}) as Box<dyn Day>),
            ]
            .into_iter()
            .collect(),
            runner_option: match (args.test, args.day) {
                (true, x) => RunnerOption::Test(x),
                (false, x) => RunnerOption::Task(x),
            },
        }
    }
    pub fn run(&self) {
        match &self.runner_option {
            RunnerOption::Task(Some(day)) | RunnerOption::Test(Some(day)) => self.run_day(*day),
            RunnerOption::Task(None) | RunnerOption::Test(None) => {
                for (day, _) in self.days.iter() {
                    self.run_day(*day);
                }
            }
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
                1 => println!(
                    "Part 1: {}",
                    day.part1(format!(
                        "inputs/d{}.{}",
                        d,
                        if let RunnerOption::Test(_) = self.runner_option {
                            "test"
                        } else {
                            "task"
                        }
                    ))
                ),
                2 => println!(
                    "Part 2: {}",
                    day.part2(format!(
                        "inputs/d{}.{}",
                        d,
                        if let RunnerOption::Test(_) = self.runner_option {
                            "test"
                        } else {
                            "task"
                        }
                    ))
                ),
                _ => println!("Invalid part number"),
            }
        } else {
            println!("Day {} not found", d);
        }
    }
}
