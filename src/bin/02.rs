advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines()

    .filter_map(|line| { let split_line =line.split_ascii_whitespace()
        .map(|item| item.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

        if split_line.windows(2).all(|w| matches!(w[1] - w[0], 1..=3)) ||
           split_line.windows(2).all(|w| matches!(w[0] - w[1], 1..=3))
            {Some(true)}
        else
            {None}
        }
    )
    .count() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
