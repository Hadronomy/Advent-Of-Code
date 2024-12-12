use miette::*;
use std::collections::{HashMap, HashSet, VecDeque};
use strum::{EnumIter, IntoEnumIterator};

pub fn process(input: &str) -> Result<String> {
    let map = parse_map(input);
    let grid = Grid::new(&map);
    let total_price = calculate_total_price(&grid);
    Ok(total_price.to_string())
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl From<Direction> for Point {
    fn from(dir: Direction) -> Self {
        dir.to_point()
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

#[derive(Clone)]
struct Grid {
    map: Vec<Vec<char>>,
    height: isize,
    width: isize,
}

impl Grid {
    fn new(input: &[Vec<char>]) -> Self {
        let height = input.len() as isize;
        let width = input[0].len() as isize;
        Self {
            map: input.to_vec(),
            height,
            width,
        }
    }

    fn get(&self, point: Point) -> Option<char> {
        if point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height {
            Some(self.map[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    fn adjacency(&self, point: Point, plant_type: char) -> Vec<Point> {
        Direction::iter()
            .map(|dir| point + dir.to_point())
            .filter(|&p| self.get(p) == Some(plant_type))
            .collect()
    }
}

#[derive(EnumIter, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_point(self) -> Point {
        match self {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn bfs(grid: &Grid, start: Point, plant_type: char) -> (HashSet<Point>, Vec<Point>) {
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

fn calculate_total_price(grid: &Grid) -> usize {
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
