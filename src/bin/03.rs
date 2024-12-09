use regex::Regex;

advent_of_code::solution!(3);
use nom::{
    bytes::complete::{tag, take},
    character::complete::{char, digit1},
    combinator::map_res,
    multi::many0,
    sequence::{delimited, separated_pair},
    IResult,
};

fn parse_int(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn parse_mul(input: &str) -> IResult<&str, (i32, i32)> {
    delimited(
        tag("mul("),
        separated_pair(parse_int, char(','), parse_int),
        char(')'),
    )(input)
}

fn parse_tokens<'a>(input: &'a str) -> IResult<&'a str, Vec<(i32, i32)>> {
    let mut inside_dont = false;

    let mut parser = many0(
        move |input: &'a str| -> IResult<&'a str, Option<(i32, i32)>> {
            if let Ok((remaining, _)) = tag::<&str, &str, nom::error::Error<&str>>("don't()")(input)
            {
                inside_dont = true;
                Ok((remaining, None))
            } else if let Ok((remaining, _)) =
                tag::<&str, &str, nom::error::Error<&str>>("do()")(input)
            {
                inside_dont = false;
                Ok((remaining, None))
            } else if let Ok((remaining, mul)) = parse_mul(input) {
                if inside_dont {
                    Ok((remaining, None))
                } else {
                    Ok((remaining, Some(mul)))
                }
            } else {
                // Ensure we always consume at least one character if nothing else matches
                let (remaining, _) = take(1usize)(input)?;
                Ok((remaining, None))
            }
        },
    );

    let (remaining, results) = parser(input)?;
    let results: Vec<(i32, i32)> = results.into_iter().flatten().collect();
    Ok((remaining, results))
}

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((?P<x>[0-9]{1,3}),(?P<y>[0-9]{1,3})\)").unwrap();
    Some(
        input
            .lines()
            .map(|line| {
                regex
                    .captures_iter(line)
                    .map(|cap| {
                        // within the match, find the group right and group left and extract the value
                        cap["x"].parse::<u32>().unwrap() * cap["y"].parse::<u32>().unwrap()
                    })
                    .sum::<u32>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    match parse_tokens(input) {
        Ok((_, parsed_mul)) => {
            let sum: u32 = parsed_mul
                .into_iter()
                .map(|(x, y)| x as u32 * y as u32)
                .sum();
            Some(sum)
        }
        Err(e) => {
            println!("Parser error: {:?}", e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
