use miette::*;
use std::collections::HashSet;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut total_price = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if !visited[i][j] {
                let region = explore_region(&map, &mut visited, i, j);
                let area = region.len();
                let perimeter = calculate_perimeter(&map, &region);
                total_price += area * perimeter; // Assuming some logic to calculate price
            }
        }
    }

    Ok(total_price.to_string())
}

fn explore_region(
    map: &[Vec<char>],
    visited: &mut [Vec<bool>],
    start_x: usize,
    start_y: usize,
) -> HashSet<(usize, usize)> {
    let mut stack = vec![(start_x, start_y)];
    let mut region = HashSet::new();
    let plant_type = map[start_x][start_y];
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((x, y)) = stack.pop() {
        if x < map.len() && y < map[0].len() && !visited[x][y] && map[x][y] == plant_type {
            visited[x][y] = true;
            region.insert((x, y));
            for (dx, dy) in &directions {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 {
                    stack.push((nx as usize, ny as usize));
                }
            }
        }
    }

    region
}

fn calculate_perimeter(map: &[Vec<char>], region: &HashSet<(usize, usize)>) -> usize {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut perimeter = 0;

    for &(x, y) in region {
        for (dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0
                || ny < 0
                || nx >= map.len() as isize
                || ny >= map[0].len() as isize
                || !region.contains(&(nx as usize, ny as usize))
            {
                perimeter += 1;
            }
        }
    }

    perimeter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
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
        assert_eq!("1930", process(input)?);
        Ok(())
    }
}
