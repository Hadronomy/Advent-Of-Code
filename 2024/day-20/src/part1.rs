use glam::IVec2;
use miette::*;
use std::collections::{hash_map, HashMap, HashSet, VecDeque};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start = Position::ZERO;
    let mut end = Position::ZERO;

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = Position::new(x as i32, y as i32);
            } else if cell == 'E' {
                end = Position::new(x as i32, y as i32);
            }
        }
    }

    let start_distance = bfs(&grid, start);
    let end_distance = bfs(&grid, end);
    let final_distance = start_distance[&end];

    let mut skips = 0;
    let mut saved_counts = HashMap::new();
    let walkable_cells: Vec<Position> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &cell)| cell != '#')
                .map(move |(x, _)| Position::new(x as i32, y as i32))
        })
        .collect();

    for &point1 in walkable_cells.iter() {
        for &point2 in walkable_cells.iter() {
            let skip_distance = manhattan_distance(point1, point2);
            if skip_distance > 2 {
                continue;
            }
            let nd = start_distance[&point1] + skip_distance + end_distance[&point2];
            let saved = final_distance - nd;
            if saved > 0 {
                *saved_counts.entry(saved).or_insert(0) += 1;
                if saved >= 100 {
                    skips += 1;
                }
            }
        }
    }

    Ok(skips.to_string())
}

type Position = IVec2;

fn manhattan_distance(a: Position, b: Position) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn get_neighbors(
    position: Position,
    grid: &[Vec<char>],
    visited: &HashSet<Position>,
) -> Vec<Position> {
    let mut neighbors = Vec::new();
    let directions = [
        IVec2::new(-1, 0),
        IVec2::new(1, 0),
        IVec2::new(0, -1),
        IVec2::new(0, 1),
    ];

    for direction in directions {
        let next = position + direction;
        if next.x >= 0
            && next.x < grid[0].len() as i32
            && next.y >= 0
            && next.y < grid.len() as i32
            && grid[next.y as usize][next.x as usize] != '#'
            && !visited.contains(&next)
        {
            neighbors.push(next);
        }
    }
    neighbors
}

fn bfs(grid: &[Vec<char>], start: Position) -> HashMap<Position, i32> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(start);
    distances.insert(start, 0);

    while let Some(pos) = queue.pop_front() {
        let current_distance = distances[&pos];
        let neighbors = get_neighbors(pos, grid, &visited);

        for neighbor in neighbors {
            if let hash_map::Entry::Vacant(entry) = distances.entry(neighbor) {
                entry.insert(current_distance + 1);
                queue.push_back(neighbor);
                visited.insert(neighbor);
            }
        }
    }

    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!("0", process(input)?);
        Ok(())
    }
}
