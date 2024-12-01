macro_rules! generate_day_tests {
    ($day_num:expr, $part1_expected:expr, $part2_expected:expr) => {
        #[cfg(test)]
        pub mod tests {
            use super::*;

            #[test]
            fn test_part1() {
                let day = paste::paste! { [<Day $day_num>] {} };
                assert_eq!(
                    day.part1(format!("inputs/d{}.test", $day_num)),
                    $part1_expected.to_string()
                );
            }

            #[test]
            fn test_part2() {
                let day = paste::paste! { [<Day $day_num>] {} };
                assert_eq!(
                    day.part2(format!("inputs/d{}.test", $day_num)),
                    $part2_expected.to_string()
                );
            }
        }
    };
}
