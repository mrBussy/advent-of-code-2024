
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {

    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    input.lines().for_each(|line| {
        if line.contains("|") {
            rules.push((line.split('|').next().unwrap().parse::<i32>().unwrap(), line.split('|').last().unwrap().parse::<i32>().unwrap()));
        }
        else if line.contains(",") {
            updates.push(line.split(",").map(|item| item.parse::<i32>().unwrap()).collect::<Vec<i32>>());
        }
    });

    Some(
        updates.into_iter().filter(|update: &Vec<i32>| {
            rules.iter().all(|rule| {
                let left_index = update.iter().position(|item| rule.0 == *item);
                let right_index = update.iter().position(|item| rule.1 == *item);

                if let (Some(left), Some(right)) = (left_index, right_index) {
                    left < right
                }
                else { true }
            })
        })
        .map(|updates|  updates[updates.len()/2])
        .sum::<i32>() as u32
    )
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
