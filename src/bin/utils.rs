
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
