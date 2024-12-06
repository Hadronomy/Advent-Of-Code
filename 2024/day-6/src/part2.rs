use indicatif::ParallelProgressIterator;
use miette::*;
use rayon::prelude::*;
use std::collections::HashSet;

pub fn process(input: &str) -> Result<String> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (position, direction) = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &cell)| ((x as isize, y as isize), cell))
        })
        .find(|&(_, cell)| cell == '^')
        .map(|(pos, _)| (pos, Direction::Up))
        .ok_or_else(|| miette!("Initial position not found"))?;

    let start_position = position;
    let (_, visited) = simulate(&map, position, direction);

    let map_clone = map.clone();
    let loop_amount: usize = visited
        .iter()
        .cloned()
        .collect::<Vec<_>>()
        .par_iter()
        .progress_count(visited.len() as u64)
        .map(|&(x, y)| {
            let mut map_clone = map_clone.clone();
            map_clone[y as usize][x as usize] = 'O';
            let (looped, _) = simulate(&map_clone, start_position, direction);
            map_clone[y as usize][x as usize] = '.';
            if looped {
                1
            } else {
                0
            }
        })
        .sum();

    Ok(loop_amount.to_string())
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_forward(self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }
}

pub fn simulate(
    map: &[Vec<char>],
    position: (isize, isize),
    direction: Direction,
) -> (bool, HashSet<(isize, isize)>) {
    let mut visited: HashSet<((isize, isize), Direction)> = HashSet::new();
    let mut position = position;
    let mut direction = direction;

    loop {
        let next_position = direction.move_forward(position);

        if next_position.1 < 0
            || next_position.1 >= map.len() as isize
            || next_position.0 < 0
            || next_position.0 >= map[0].len() as isize
        {
            visited.insert((position, direction));
            return (false, visited.iter().map(|&(pos, _)| pos).collect());
        }

        if visited.contains(&(position, direction)) {
            return (true, visited.iter().map(|&(pos, _)| pos).collect());
        }
        visited.insert((position, direction));

        if map[next_position.1 as usize][next_position.0 as usize] == '#'
            || map[next_position.1 as usize][next_position.0 as usize] == 'O'
        {
            direction = direction.turn_right();
        } else {
            position = next_position;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
