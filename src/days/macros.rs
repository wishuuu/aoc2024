macro_rules! generate_day_tests {
    ($day_num:expr, $part1_test_expected:expr, $part2_test_expected:expr, $part1_task_expected:expr, $part2_task_expected:expr) => {
        #[cfg(test)]
        pub mod tests {
            use super::*;

            #[test]
            fn test_part1() {
                let day = paste::paste! { [<Day $day_num>] {} };
                assert_eq!(
                    day.part1(format!("inputs/d{}.test", $day_num)),
                    $part1_test_expected.to_string()
                );
            }

            #[test]
            fn test_part2() {
                let day = paste::paste! { [<Day $day_num>] {} };
                assert_eq!(
                    day.part2(format!("inputs/d{}.test", $day_num)),
                    $part2_test_expected.to_string()
                );
            }

            #[test]
            fn task_part1() {
                let day = paste::paste! { [<Day $day_num>] {} };
                assert_eq!(
                    day.part1(format!("inputs/d{}.task", $day_num)),
                    $part1_task_expected.to_string()
                );
            }

            #[test]
            fn task_part2() {
                let day = paste::paste! { [<Day $day_num>] {} };
                assert_eq!(
                    day.part2(format!("inputs/d{}.task", $day_num)),
                    $part2_task_expected.to_string()
                );
            }
        }
    };
}
