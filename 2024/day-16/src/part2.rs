use glam::IVec2;
use miette::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn process(input: &str) -> Result<String> {
    let grid: Vec<Vec<CellType>> = parse_map(input);
    let start = find_position(&grid, CellType::Start).unwrap();
    let end = find_position(&grid, CellType::End).unwrap();

    let result = find_best_paths(&grid, start, end);
    Ok(result.len().to_string())
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

    fn from_index(index: usize) -> Self {
        match index {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("Invalid direction index"),
        }
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

fn find_best_paths(grid: &[Vec<CellType>], start: IVec2, end: IVec2) -> HashSet<IVec2> {
    let adj = |state: &(IVec2, usize)| -> Vec<((IVec2, usize), usize)> {
        let (position, direction) = *state;
        let mut neighbors = Vec::new();

        // Move forward
        let new_position = position + Direction::from_index(direction).to_ivec2();
        if is_valid_move(grid, new_position) {
            neighbors.push(((new_position, direction), 1));
        }

        // Rotate clockwise
        let clockwise_d = (direction + 1) % 4;
        neighbors.push(((position, clockwise_d), 1000));

        // Rotate counterclockwise
        let ccw_d = (direction + 3) % 4;
        neighbors.push(((position, ccw_d), 1000));

        neighbors
    };

    let mut distances: HashMap<(IVec2, usize), usize> = HashMap::new();
    let mut prevs: HashMap<(IVec2, usize), Vec<(IVec2, usize)>> = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(((start, 0), 0));
    distances.insert((start, 0), 0);

    while let Some(((current, direction), current_cost)) = queue.pop_front() {
        for (next_state, edge_cost) in adj(&(current, direction)) {
            let new_cost = current_cost + edge_cost;
            if !distances.contains_key(&next_state) || new_cost < distances[&next_state] {
                distances.insert(next_state, new_cost);
                prevs.insert(next_state, vec![(current, direction)]);
                queue.push_back((next_state, new_cost));
            } else if new_cost == distances[&next_state] {
                prevs.get_mut(&next_state).unwrap().push((current, direction));
            }
        }
    }

    let min_cost = distances.iter()
        .filter(|((position, _), _)| *position == end)
        .map(|(_, &cost)| cost)
        .min()
        .unwrap_or(usize::MAX);

    let mut best_paths = HashSet::new();
    let mut bfs_queue = VecDeque::new();

    for i in 0..4 {
        if let Some(&cost) = distances.get(&(end, i)) {
            if cost == min_cost {
                bfs_queue.push_back((end, i));
            }
        }
    }

    while let Some(state) = bfs_queue.pop_front() {
        let (position, _) = state;
        best_paths.insert(position);
        if let Some(prev_states) = prevs.get(&state) {
            for &prev_state in prev_states {
                bfs_queue.push_back(prev_state);
            }
        }
    }

    best_paths
}

fn is_valid_move(grid: &[Vec<CellType>], pos: IVec2) -> bool {
    (pos.y as usize) < grid.len()
        && (pos.x as usize) < grid[0].len()
        && grid[pos.y as usize][pos.x as usize] != CellType::Wall
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
        assert_eq!(process(input)?, "45");
        Ok(())
    }
}
