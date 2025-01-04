use std::collections::HashMap;

advent_of_code::solution!(6);

#[derive(Hash, Debug, Clone, Copy)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Coordinate {}

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

fn walk(floorplan: HashMap<Coordinate, char>) -> Option<HashMap<Coordinate, char>> {
    let mut floorplan = floorplan;
    let starting_point:Coordinate = *floorplan.iter().find(|entry| *entry.1=='^').unwrap().0;
    let mut active_point:Coordinate = starting_point;
    let mut direction = Direction::North;
    let mut steps=0;

    while floorplan.contains_key(&active_point) && steps<140 {
        floorplan.entry(active_point).and_modify(|e| *e='$');
        let mut next_point: Coordinate = active_point.move_in_direction(&direction);

        let next: Option<&char> = floorplan.get(&next_point);
        match next {
            Some('#') | Some('0') => { direction = match direction {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::East => Direction::South,
            };
            next_point = active_point.move_in_direction(&direction);},
            _ => ()
        }

        active_point = next_point;
        steps+=1;
    }

    if steps<140 {
        Some(floorplan)
    }
    else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {

    let floorplan = walk(input.lines().enumerate().flat_map( |(y, line)| {
        line.chars().enumerate().map(move|(x, c)|
        (Coordinate::new(x as i32, y as i32), c)
    )}).collect::<HashMap<Coordinate, char>>()).unwrap();

    Some(
        floorplan.iter().filter(|entry| *entry.1=='$').count() as u32
    )
}
pub struct Floorplan(HashMap<Coordinate, char>);

impl std::fmt::Debug for Floorplan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let array: Vec<Vec<char>> = (0..=self.0.keys().map(|c| c.y).max().unwrap()).map(|y| {
                (0..=self.0.keys().map(|c| c.x).max().unwrap()).map(|x| {
                    *self.0.get(&Coordinate::new(x, y)).unwrap_or(&' ')
                }).collect()
            }).collect();

            array.iter().for_each(|line|
                line.iter().for_each(|item| write!(f, "{}", item).unwrap() )
            );
            Ok(())
    }
}

pub fn part_two(input: &str) -> Option<u32> {
/* 
    let floorplan = input.lines().enumerate().flat_map( |(y, line)| {
        line.chars().enumerate().map(move|(x, c)|
        (Coordinate::new(x as i32, y as i32), c)
    )}).collect::<HashMap<Coordinate, char>>();
    let starting_point:Coordinate = *floorplan.iter().find(|entry| *entry.1=='^').unwrap().0;

    let mut visited_floorplan:Floorplan = walk(floorplan).unwrap() as Floorplan;
    let mut possibilities: u32 = 0;
    visited_floorplan.entry(starting_point).and_modify(|e| *e='^');

    visited_floorplan.iter().filter(|entry| *entry.1=='$').for_each(|entry| {
            let mut tmp_floorplan: Floorplan = visited_floorplan.clone();
            tmp_floorplan.entry(*entry.0).and_modify(|e| *e='0');
            println!("Starting at {:?} with floorplan {:?}", entry.0, tmp_floorplan);

            if let Some(_) = walk(tmp_floorplan) {
                possibilities+=1;
            }
        }
    );
*/
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
        //assert_eq!(result, Some(6));
        assert_eq!(result, None);
    }
}
