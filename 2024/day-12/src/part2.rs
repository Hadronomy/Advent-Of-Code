use miette::*;
use std::collections::{HashMap, HashSet, VecDeque};
use strum::{EnumIter, IntoEnumIterator};

pub fn process(input: &str) -> Result<String> {
    let map = parse_map(input);
    let grid = Grid::new(&map);
    let total_price = calculate_total_price(&grid);
    Ok(total_price.to_string())
}

#[derive(Clone)]
pub struct Grid {
    map: Vec<Vec<char>>,
    height: isize,
    width: isize,
}

impl Grid {
    pub fn new(input: &[Vec<char>]) -> Self {
        let height = input.len() as isize;
        let width = input[0].len() as isize;
        Self {
            map: input.to_vec(),
            height,
            width,
        }
    }

    pub fn get(&self, point: Point) -> Option<char> {
        if point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height {
            Some(self.map[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    pub fn adjacency(&self, point: Point, plant_type: char) -> Vec<Point> {
        Direction::iter()
            .map(|dir| point + dir.into())
            .filter(|&p| self.get(p) == Some(plant_type))
            .collect()
    }
}

#[derive(EnumIter, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    x: isize,
    y: isize,
}

impl From<Direction> for Point {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

///
/// Performs a Breadth-First Search (BFS) on the grid starting from the given point and searching for the specified plant type.
///
/// # Arguments
///
/// * `grid` - A reference to the grid.
/// * `start` - The starting point for the BFS.
/// * `plant_type` - The type of plant to search for.
///
/// # Returns
///
/// A tuple containing:
/// * A `HashSet` of visited points.
/// * A `Vec` of points in the region.
///
pub fn bfs(grid: &Grid, start: Point, plant_type: char) -> (HashSet<Point>, Vec<Point>) {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut region = Vec::new();

    while let Some(p) = queue.pop_front() {
        if !visited.insert(p) {
            continue;
        }
        region.push(p);
        for neighbor in grid.adjacency(p, plant_type) {
            if !visited.contains(&neighbor) {
                queue.push_back(neighbor);
            }
        }
    }

    (visited, region)
}

///
/// Calculates the total price of all plants in the grid.
///
/// # Algorithm
///
/// 1. For each cell in the grid:
///    - 1.1. If the cell has not been visited:
///        - 1.1.1. Perform a Breadth-First Search ([`bfs`]) starting from the cell and searching for the plant type.
///               - The [`bfs`] function explores all connected cells of the same plant type starting from the given cell.
///               - It returns a set of visited points and a vector of points in the region.
///        - 1.1.2. Calculate the perimeter of the region using the [`bfs`] result.
///               - For each point in the region, check its neighbors in all directions.
///               - If a neighbor is not part of the region, it contributes to the perimeter.
///               - Use a HashMap to count the number of times each perimeter point is encountered.
///        - 1.1.3. Calculate the area of the region.
///               - The area is simply the number of points in the region.
///        - 1.1.4. Calculate the price of the region (area * perimeter).
///               - Multiply the area by the perimeter to get the price of the region.
/// 2. Return the total price of all regions.
///    - Sum the prices of all regions to get the total price.
///
/// # Examples
///
/// ```
/// use aoc2024_day_12::part2::{Grid, calculate_total_price};
///
/// let grid = Grid::new(&vec![
///     vec!['A', 'A', 'B', 'B', 'B'],
///     vec!['A', 'A', 'B', 'C', 'C'],
///     vec!['D', 'D', 'B', 'C', 'C'],
///     vec!['D', 'D', 'E', 'E', 'E'],
///     vec!['F', 'F', 'E', 'G', 'G'],
/// ]);
/// let total_price = calculate_total_price(&grid);
/// assert_eq!(118, total_price);
/// ```
///
pub fn calculate_total_price(grid: &Grid) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut total_price = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point { x, y };
            if visited.contains(&p) {
                continue;
            }

            let plant_type = grid.get(p).unwrap();
            let (_, region) = bfs(grid, p, plant_type);

            let mut perim_points = HashMap::new();
            for &point in &region {
                for direction in Direction::iter().map(|d| d.into()) {
                    let neighbor = point + direction;
                    if !region.contains(&neighbor) {
                        *perim_points.entry((neighbor, direction)).or_insert(0) += 1;
                    }
                }
            }
            visited.extend(&region);

            let mut sides = 0;
            let mut perim_points_set: HashSet<_> = perim_points.keys().cloned().collect();
            while let Some(&(p, d)) = perim_points_set.iter().next() {
                perim_points_set.remove(&(p, d));
                sides += 1;

                let mut stack = vec![p];
                while let Some(perimeter_point) = stack.pop() {
                    for direction in Direction::iter().map(|d| d.into()) {
                        let np = perimeter_point + direction;
                        if perim_points_set.remove(&(np, d)) {
                            stack.push(np);
                        }
                    }
                }
            }

            total_price += sides * region.len();
        }
    }

    total_price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1206", process(input)?);
        Ok(())
    }

    #[test]
    fn test_small_case() -> Result<()> {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!("236", process(input)?);
        Ok(())
    }
}
