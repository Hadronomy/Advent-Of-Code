use miette::*;
use std::collections::{HashSet, VecDeque};

const GRID_SIZE: usize = 71;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        let mut neighbors = Vec::new();
        if self.x > 0 {
            neighbors.push(Point {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.x < GRID_SIZE - 1 {
            neighbors.push(Point {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            neighbors.push(Point {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.y < GRID_SIZE - 1 {
            neighbors.push(Point {
                x: self.x,
                y: self.y + 1,
            });
        }
        neighbors
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<usize> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Point {
                x: parts[0],
                y: parts[1],
            }
        })
        .collect()
}

fn is_path_possible(grid: &[Vec<bool>], start: Point, end: Point) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((current, steps)) = queue.pop_front() {
        if current == end {
            return Some(steps);
        }

        for neighbor in current.neighbors() {
            if !visited.contains(&neighbor) && !grid[neighbor.x][neighbor.y] {
                visited.insert(neighbor);
                queue.push_back((neighbor, steps + 1));
            }
        }
    }

    None
}

pub fn process(input: &str) -> Result<String> {
    let points = parse_input(input);

    let mut grid = vec![vec![false; GRID_SIZE]; GRID_SIZE];
    for point in points.iter().take(1024) {
        grid[point.x][point.y] = true;
    }

    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: GRID_SIZE - 1,
        y: GRID_SIZE - 1,
    };

    let result = is_path_possible(&grid, start, end)
        .ok_or_else(|| miette!("No path found from start to end"))?;

    Ok(result.to_string())
}
