use super::Day;
use crate::utils::file::read_lines;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Day4 {}

#[derive(Debug, PartialEq, EnumIter)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    fn as_coord(&self) -> (i32, i32) {
        match self {
            Direction::Up => (1, 0),
            Direction::UpRight => (1, 1),
            Direction::Right => (0, 1),
            Direction::DownRight => (-1, 1),
            Direction::Down => (-1, 0),
            Direction::DownLeft => (-1, -1),
            Direction::Left => (0, -1),
            Direction::UpLeft => (1, -1),
        }
    }
}

impl Day for Day4 {
    fn part1(&self, input_path: String) -> String {
        let lines = read_lines(input_path);
        let hight = lines.len();
        let width = lines[0].len();
        let sequence = ['X', 'M', 'A', 'S'];

        let is_valid = |x: i32, y: i32, sequence_index: i32, direction: &Direction| -> bool {
            let (dx, dy) = direction.as_coord();
            let x = x + dx * sequence_index;
            let y = y + dy * sequence_index;
            x >= 0
                && x < hight as i32
                && y >= 0
                && y < width as i32
                && lines[x as usize].chars().nth(y as usize).unwrap_or(' ')
                    == sequence[sequence_index as usize]
        };

        let mut count = 0;
        for i in 0..lines.len() {
            let line = &lines[i];
            for j in 0..line.len() {
                for dir in Direction::iter() {
                    if is_valid(i as i32, j as i32, 0, &dir)
                        && is_valid(i as i32, j as i32, 1, &dir)
                        && is_valid(i as i32, j as i32, 2, &dir)
                        && is_valid(i as i32, j as i32, 3, &dir)
                    {
                        count += 1;
                    }
                }
            }
        }
        count.to_string()
    }

    fn part2(&self, input_path: String) -> String {
        let lines = read_lines(input_path);

        let get_alligned_char =
            |x: usize, y: usize, direction: Option<Direction>| -> Option<char> {
                let (dx, dy) = if let Some(direction) = direction {
                    direction.as_coord()
                } else {
                    (0, 0)
                };
                let x = x as i32 + dx;
                let y = y as i32 + dy;
                lines.get(x as usize)?.chars().nth(y as usize).to_owned()
            };

        let mut count = 0;
        for i in 0..lines.len() {
            let line = &lines[i];
            for j in 0..line.len() {
                if get_alligned_char(i, j, None) == Some('A') {
                    let diag1 = matches!(
                        (
                            get_alligned_char(i, j, Some(Direction::UpRight)),
                            get_alligned_char(i, j, Some(Direction::DownLeft))
                        ),
                        (Some('S'), Some('M')) | (Some('M'), Some('S'))
                    );
                    let diag2 = matches!(
                        (
                            get_alligned_char(i, j, Some(Direction::UpLeft)),
                            get_alligned_char(i, j, Some(Direction::DownRight))
                        ),
                        (Some('S'), Some('M')) | (Some('M'), Some('S'))
                    );
                    if diag1 && diag2 {
                        count += 1;
                    }
                }
            }
        }
        count.to_string()
    }
}

generate_day_tests!(4, "18", "9", "2562", "1902");
