advent_of_code::solution!(1);

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

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
