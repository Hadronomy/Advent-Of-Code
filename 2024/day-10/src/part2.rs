use miette::*;
use std::collections::HashSet;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let mut total_rating = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            if height == 0 {
                total_rating += count_distinct_trails(&map, i, j, &mut HashSet::new());
            }
        }
    }

    Ok(total_rating.to_string())
}

fn count_distinct_trails(map: &[Vec<u8>], i: usize, j: usize, visited: &mut HashSet<(usize, usize)>) -> usize {
    if map[i][j] == 9 {
        return 1;
    }

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut trail_count = 0;

    visited.insert((i, j));

    for &(di, dj) in &directions {
        let ni = i as isize + di;
        let nj = j as isize + dj;

        if ni >= 0 && ni < map.len() as isize && nj >= 0 && nj < map[0].len() as isize {
            let ni = ni as usize;
            let nj = nj as usize;

            if !visited.contains(&(ni, nj)) && map[ni][nj] == map[i][j] + 1 {
                trail_count += count_distinct_trails(map, ni, nj, visited);
            }
        }
    }

    visited.remove(&(i, j));

    trail_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("81", process(input)?);
        Ok(())
    }
}
