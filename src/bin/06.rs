use std::collections::HashMap;

advent_of_code::solution!(6);

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {

    pub fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }

    pub fn up(self) -> Self {
        Coordinate::new(self.x, self.y - 1)
    }
    pub fn down(self) -> Self {
        Coordinate::new(self.x, self.y + 1)
    }
    pub fn left(self) -> Self {
        Coordinate::new(self.x-1, self.y)
    }
    pub fn right(self) -> Self {
        Coordinate::new(self.x+1, self.y)
    }
    pub fn move_in_direction(self, direction: &Direction) -> Self {
        match direction {
            Direction::North => self.up(),
            Direction::South => self.down(),
            Direction::West => self.left(),
            Direction::East => self.right(),
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    West ,
    East,
}

pub fn part_one(input: &str) -> Option<u32> {

    let mut floorplan =  input.lines().enumerate().flat_map( |(y, line)| {
        line.chars().enumerate().map(move|(x, c)|
        (Coordinate::new(x as i32, y as i32), c)
    )}).collect::<HashMap<Coordinate, char>>();

    let starting_point:Coordinate = *floorplan.iter().find(|entry| *entry.1=='^').unwrap().0;
    let mut active_point:Coordinate = starting_point;
    let mut direction = Direction::North;

    while floorplan.contains_key(&active_point) {
        floorplan.entry(active_point).and_modify(|e| *e='$');
        let mut next_point: Coordinate = active_point.move_in_direction(&direction);

        if let Some('#') = floorplan.get(&next_point) {
            direction = match direction {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::East => Direction::South,
            };
            next_point = active_point.move_in_direction(&direction);
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
