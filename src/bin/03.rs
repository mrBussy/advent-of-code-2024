use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((?P<x>[0-9]{1,3}),(?P<y>[0-9]{1,3})\)").unwrap();
    Some(input.lines().map(|line|{
        regex
        .captures_iter(line)
        .map(|cap| {
            // within the match, find the group right and group left and extract the value
            cap["x"].parse::<u32>().unwrap()
            * cap["y"].parse::<u32>().unwrap()
        }).sum::<u32>()
    }).sum())
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
