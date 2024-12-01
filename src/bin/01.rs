advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list_left: Vec<u32> = Vec::new();
    let mut list_right: Vec<u32> = Vec::new();

    input.split('\n').for_each(|line| {
        let split_line = line.split_ascii_whitespace().collect::<Vec<&str>>();
        list_left.push(split_line[0].parse::<u32>().unwrap());
        list_right.push(split_line[1].parse::<u32>().unwrap());
    });
    list_left.sort();
    list_right.sort();

    let mut list_result: Vec<u32> = Vec::new();
    for i in 0..list_left.len() {
        list_result.push(list_left[i].abs_diff(list_right[i]));
    }

    Some(list_result.into_iter().sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list_left: Vec<u32> = Vec::new();
    let mut list_right: Vec<u32> = Vec::new();

    input.split('\n').for_each(|line| {
        let split_line = line.split_ascii_whitespace().collect::<Vec<&str>>();
        list_left.push(split_line[0].parse::<u32>().unwrap());
        list_right.push(split_line[1].parse::<u32>().unwrap());
    });

    let mut result: u32 = 0;

    list_left.into_iter().for_each(|left| {
        result += left
            * list_right
                .clone()
                .into_iter()
                .filter(|right| right.eq(&left))
                .count() as u32;
    });

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
