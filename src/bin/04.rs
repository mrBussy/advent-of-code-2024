use std::collections::HashMap;

advent_of_code::solution!(4);

pub fn count_horizontal(matrix: Vec<Vec<char>>) -> Option<u32> {
    let xmas: [char; 4] = ['X', 'M', 'A', 'S'];
    let xmas_rev: [char; 4] = ['S', 'A', 'M', 'X'];

    Some(matrix.iter().map(|line|
        line.windows(4).filter_map(|slize| if slize.eq(&xmas) || slize.eq(&xmas_rev) {Some(true)} else {None}).count() as u32
    ).sum())

    //let horizontal: Vec<bool> = matrix.windows(4).filter_map(|slize| if slize.eq(&xmas) || slize.eq(&xmas_rev) {Some(true)} else {None}).collect();


}

pub fn count_vertical(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let matrix_width = matrix[0].len();
    let matrix_height = matrix.len();

    // Create a new transposed matrix
    let transposed: Vec<Vec<char>> = (0..matrix_width)
        .map(|col| {
            (0..matrix_height)
                .map(|row| matrix[row][col])
                .collect::<Vec<char>>()
        })
    .collect();

    count_horizontal(transposed)
}

pub fn count_diagonal(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = matrix.len();
    let cols = matrix[0].len();
    let word_chars: Vec<char> = "XMAS".chars().collect();
    let word_chars_rev: Vec<char> = "XMAS".chars().rev().collect();
    let word_len = word_chars.len();
    let mut count: u32 = 0;

     // Top-left to bottom-right diagonals
    for i in 0..rows {
        for j in 0..cols {
            if i + word_len <= rows && j + word_len <= cols {
                // Forward check
                let mut found_forward = true;
                let mut found_reverse = true;

                for k in 0..word_len {
                    // Check forward pattern
                    if matrix[i + k][j + k] != word_chars[k] {
                        found_forward = false;
                    }
                    // Check reverse pattern
                    if matrix[i + k][j + k] != word_chars_rev[k] {
                        found_reverse = false;
                    }
                    // Early exit if both patterns fail
                    if !found_forward && !found_reverse {
                        break;
                    }
                }
                if found_forward {
                    count += 1;
                }
                if found_reverse {
                    count += 1;
                }
            }
        }
    }

    // Top-right to bottom-left diagonals
    for i in 0..rows {
        for j in (word_len-1)..cols {
            if i + word_len <= rows && j >= word_len-1 {
                // Forward check
                let mut found_forward = true;
                let mut found_reverse = true;

                for k in 0..word_len {
                    // Check forward pattern
                    if matrix[i + k][j - k] != word_chars[k] {
                        found_forward = false;
                    }
                    // Check reverse pattern
                    if matrix[i + k][j - k] != word_chars_rev[k] {
                        found_reverse = false;
                    }
                    // Early exit if both patterns fail
                    if !found_forward && !found_reverse {
                        break;
                    }
                }
                if found_forward {
                    count += 1;
                }
                if found_reverse {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_one(input: &str) -> Option<u32> {

    Some(
        count_horizontal(input.lines().into_iter().map(|line| line.chars().collect()).collect()).unwrap() +
        count_vertical(input).unwrap() +
        count_diagonal(input).unwrap()
    )
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }
}

pub fn part_two(input: &str) -> Option<u32> {

    let positions =  input.lines().enumerate().flat_map(move |(y, line)| {
        line.chars().enumerate().map(move|(x, c)|
        (Coordinate{x: x as i32, y: y as i32}, c)
    )}).collect::<HashMap<Coordinate, char>>();

    Some(positions.iter().filter_map(|(k, v)|
        {

            if *v == 'A' &&
                ((positions.get(&Coordinate::new(k.x - 1, k.y - 1)).unwrap_or(&char::default()) == &'M' &&
                positions.get(&Coordinate::new(k.x + 1, k.y + 1)).unwrap_or(&char::default()) == &'S') ||
                (positions.get(&Coordinate::new(k.x - 1, k.y - 1)).unwrap_or(&char::default()) == &'S' &&
                positions.get(&Coordinate::new(k.x + 1, k.y + 1)).unwrap_or(&char::default()) == &'M'))
                &&
                ((positions.get(&Coordinate::new(k.x - 1, k.y + 1)).unwrap_or(&char::default()) == &'M' &&
                positions.get(&Coordinate::new(k.x + 1, k.y - 1 )).unwrap_or(&char::default()) == &'S') ||
                (positions.get(&Coordinate::new(k.x - 1, k.y + 1)).unwrap_or(&char::default()) == &'S' &&
                positions.get(&Coordinate::new(k.x + 1, k.y - 1 )).unwrap_or(&char::default()) == &'M'))
            {
                return Some(true);
            }
            None
        }
     ).count() as u32)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
