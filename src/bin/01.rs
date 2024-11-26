advent_of_code::solution!(1);
use std::collections::HashMap;

use regex::Regex;

///
///
/// For example:
///
/// 1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet
///
/// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

/// Consider your entire calibration document. What is the sum of all of the calibration values?

/// Your puzzle answer was 54968.
fn extract_first_last_digits(s: &str) -> u32 {
    let digits: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();

    match digits.len() {
        0 => 0,
        1 => digits.first().unwrap().to_digit(10).unwrap() * 10 + digits.last().unwrap().to_digit(10).unwrap(),
        _ => digits.first().unwrap().to_digit(10).unwrap() * 10 + digits.last().unwrap().to_digit(10).unwrap()
    }

}

pub fn part_one(input: &str) -> Option<u32> {

    let result = input.split('\n')
    .fold(0, |sum: u32, element| {
        sum + extract_first_last_digits(element)}
    );

    Some(result)
}

///
///
/// Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "/// digits".
/// Equipped with this new information, you now need to find the real first and last digit on each line. For /// example:
///
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
///
/// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|zero|[0-9]").unwrap();
    // create a hashset that will turn a string to u32
    let mut hm: HashMap<&str, u32> = HashMap::new();
    hm.insert("one", 1);
    hm.insert("two", 2);
    hm.insert("three", 3);
    hm.insert("four", 4);
    hm.insert("five", 5);
    hm.insert("six", 6);
    hm.insert("seven", 7);
    hm.insert("eight", 8);
    hm.insert("nine", 9);
    hm.insert("zero", 0);

    let result = input
        .split('\n')
        .map(|line| {
            let digits: Vec<u32> = re
                .captures_iter(line)
                .map(|cap| {
                    let matched = cap.get(0).unwrap().as_str();
                    if let Ok(num) = matched.parse::<u32>() {
                        num
                    } else {
                        hm[matched]
                    }
                })
                .collect();

            if digits.is_empty() {
                0
            } else {
                digits.first().unwrap() * 10 + digits.last().unwrap()
            }
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_digits_from_string_begin_end() {
        let result: u32 = extract_first_last_digits("1abc2");
        assert_eq!(result, 12);
    }
    #[test]
    fn test_extract_digits_from_string_begin_end_between() {
        let result: u32 = extract_first_last_digits("pqr3stu8vwx");
        assert_eq!(result, 38);
    }
    #[test]
    fn test_extract_digits_from_string_begin_end_more() {
        let result: u32 = extract_first_last_digits("a1b2c3d4e5f");
        assert_eq!(result, 15);
    }
    #[test]
    fn test_extract_digits_from_string_begin_end_single() {
        let result: u32 = extract_first_last_digits("treb7uchet");
        assert_eq!(result, 77);
    }
    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
