use glam::IVec2;
use miette::*;
use nom::{
    bytes::complete::take_while, character::complete::line_ending, multi::separated_list1, IResult,
};
use nom_locate::LocatedSpan;
use std::collections::{HashMap, HashSet};

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
struct Antenna {
    position: IVec2,
    frequency: char,
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (antennas, map_size) = parse_map(input)?;
    let antinodes = calculate_antinodes(&antennas, &map_size);
    Ok(antinodes.len().to_string())
}

fn parse_map(input: &str) -> Result<(Vec<Antenna>, IVec2)> {
    let (_, antennas) =
        map_parser(Span::new(input)).map_err(|_| Error::msg("Failed to parse map"))?;
    Ok(antennas)
}

fn map_parser(input: Span) -> IResult<Span, (Vec<Antenna>, IVec2)> {
    let (input, lines) = separated_list1(line_ending, take_while(|c| c != '\n'))(input)?;
    let lines: Vec<_> = lines
        .into_iter()
        .filter(|line| !line.fragment().is_empty())
        .collect();
    let mut antennas = Vec::new();
    let mut max_x = 0;
    let max_y = lines.len() as i32;

    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.fragment().chars().enumerate() {
            if c != '.' {
                antennas.push(Antenna {
                    position: IVec2::new(x as i32, y as i32),
                    frequency: c,
                });
            }
            if x as i32 > max_x {
                max_x = x as i32;
            }
        }
    }

    let map_size = IVec2::new(max_x + 1, max_y);

    Ok((input, (antennas, map_size)))
}

fn calculate_antinodes(antennas: &[Antenna], map_size: &IVec2) -> HashSet<IVec2> {
    let mut antinodes = HashSet::new();
    let mut freq_map: HashMap<char, Vec<IVec2>> = HashMap::new();

    for antenna in antennas {
        freq_map
            .entry(antenna.frequency)
            .or_default()
            .push(antenna.position);
    }

    for positions in freq_map.values() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let pos1 = positions[i];
                let pos2 = positions[j];
                let line_vector = pos2 - pos1;
                let antinode1 = pos1 - line_vector;
                let antinode2 = pos2 + line_vector;

                if is_within_map(&antinode1, map_size) {
                    antinodes.insert(antinode1);
                }
                if is_within_map(&antinode2, map_size) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }
    antinodes
}

fn is_within_map(position: &IVec2, map_size: &IVec2) -> bool {
    position.x >= 0 && position.y >= 0 && position.x < map_size.x && position.y < map_size.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
