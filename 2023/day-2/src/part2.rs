use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, PartialOrd)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Cube {
    fn count(&self) -> u32 {
        match self {
            Cube::Red(count) => *count,
            Cube::Green(count) => *count,
            Cube::Blue(count) => *count,
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            Cube::Red(_) => "red",
            Cube::Green(_) => "green",
            Cube::Blue(_) => "blue",
        }
    }
}

#[derive(Debug)]
struct Game {
    rounds: Vec<Vec<Cube>>,
}

impl Game {
    fn fewest_possible_power(&self) -> u32 {
        let map = BTreeMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        self.rounds
            .iter()
            .fold(map, |mut map, round| {
                for cube in round.iter() {
                    map.entry(cube.to_str())
                        .and_modify(|v| *v = (*v).max(cube.count()))
                        .or_insert(cube.count());
                }
                map
            })
            .values()
            .product()
    }
}

fn cube_parser(input: &str) -> IResult<&str, Cube> {
    let (input, (num, color)) = separated_pair(
        complete::u32,
        tag(" "),
        map(alpha1, |color| match color {
            "red" => Cube::Red,
            "green" => Cube::Green,
            "blue" => Cube::Blue,
            _ => panic!("unknown color: {}", color),
        }),
    )(input)?;
    Ok((input, color(num)))
}

fn round_parser(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cube_parser)(input)?;
    Ok((input, cubes))
}

fn game_parser(input: &str) -> IResult<&str, Game> {
    let (input, _id) = preceded(tag("Game "), complete::u32)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round_parser))(input)?;
    Ok((input, Game { rounds }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game_parser)(input)?;
    Ok((input, games))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let (_, games) = parse_games(input).expect("should parse games");
    let result = games
        .iter()
        .map(|game| game.fewest_possible_power())
        .sum::<u32>();
    // dbg!(games);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, process(input)?);
        Ok(())
    }
}
