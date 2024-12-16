use glam::IVec2;
use miette::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

pub fn process(input: &str) -> Result<String> {
    let grid: Vec<Vec<CellType>> = parse_map(input);
    let start = find_position(&grid, CellType::Start).unwrap();
    let end = find_position(&grid, CellType::End).unwrap();

    let result = a_star(&grid, start, end).ok_or_else(|| miette!("No path found"))?;
    Ok(result.to_string())
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: IVec2,
    direction: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.x.cmp(&other.position.x))
            .then_with(|| self.position.y.cmp(&other.position.y))
            .then_with(|| self.direction.cmp(&other.direction))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum CellType {
    Wall,
    Path,
    Start,
    End,
}

impl CellType {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(CellType::Wall),
            '.' => Some(CellType::Path),
            'S' => Some(CellType::Start),
            'E' => Some(CellType::End),
            _ => None,
        }
    }
}

impl From<char> for CellType {
    fn from(c: char) -> Self {
        match c {
            '#' => CellType::Wall,
            '.' => CellType::Path,
            'S' => CellType::Start,
            'E' => CellType::End,
            _ => panic!("Invalid cell type"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    fn to_ivec2(self) -> IVec2 {
        match self {
            Direction::East => IVec2::new(1, 0),
            Direction::South => IVec2::new(0, 1),
            Direction::West => IVec2::new(-1, 0),
            Direction::North => IVec2::new(0, -1),
        }
    }

    fn all() -> [Direction; 4] {
        [
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::North,
        ]
    }
}

impl From<Direction> for IVec2 {
    fn from(direction: Direction) -> Self {
        direction.to_ivec2()
    }
}

fn parse_map(input: &str) -> Vec<Vec<CellType>> {
    input
        .lines()
        .map(|line| line.chars().filter_map(CellType::from_char).collect())
        .collect()
}

fn find_position(grid: &[Vec<CellType>], target: CellType) -> Option<IVec2> {
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == target {
                return Some(IVec2::new(x as i32, y as i32));
            }
        }
    }
    None
}

fn a_star(grid: &[Vec<CellType>], start: IVec2, end: IVec2) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    for &direction in Direction::all().iter() {
        heap.push(State {
            cost: 0,
            position: start,
            direction,
        });
    }

    while let Some(State {
        cost,
        position,
        direction,
    }) = heap.pop()
    {
        if position == end {
            return Some(cost);
        }

        if !visited.insert((position, direction)) {
            continue;
        }

        for &new_direction in Direction::all().iter() {
            let new_cost = if new_direction == direction {
                cost + 1
            } else {
                cost + 1000 + 1
            };

            let new_position = position + new_direction.to_ivec2();

            if new_position.x >= 0
                && new_position.y >= 0
                && (new_position.y as usize) < grid.len()
                && (new_position.x as usize) < grid[0].len()
                && grid[new_position.y as usize][new_position.x as usize] != CellType::Wall
            {
                heap.push(State {
                    cost: new_cost,
                    position: new_position,
                    direction: new_direction,
                });
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let result = process(input)?;
        assert_eq!(result, "6036");
        Ok(())
    }
}
