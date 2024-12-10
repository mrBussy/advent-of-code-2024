mod utils;
use std::collections::HashMap;
use utils::Coordinate;
use utils::Direction;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {

    let mut floorplan =  input.lines().enumerate().flat_map( |(y, line)| {
        line.chars().enumerate().map(move|(x, c)|
        (Coordinate::new(x as i32, y as i32), c)
    )}).collect::<HashMap<Coordinate, char>>();

    let starting_point:Coordinate = *floorplan.iter().filter(|entry| *entry.1=='^').next().unwrap().0;
    let mut active_point:Coordinate = starting_point.clone();
    let mut direction = Direction::North;

    while floorplan.get(&active_point).is_some() {
        floorplan.entry(active_point).and_modify(|e| *e='$');
        let mut next_point = match direction {
            Direction::North => active_point.up(),
            Direction::South => active_point.down(),
            Direction::East => active_point.right(),
            Direction::West => active_point.left(),
        };
        if let Some('#') = floorplan.get(&next_point) {
            direction = match direction {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::East => Direction::South,
            };
            next_point = match direction {
                Direction::North => active_point.up(),
                Direction::South => active_point.down(),
                Direction::East => active_point.right(),
                Direction::West => active_point.left(),
            };
        };
        active_point = next_point;

    }
    Some(
        floorplan.iter().filter(|entry| *entry.1=='$').count() as u32
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
