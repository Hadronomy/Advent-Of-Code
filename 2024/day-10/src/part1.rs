use miette::*;
use std::collections::VecDeque;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let mut total_score = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            if height == 0 {
                total_score += bfs(&map, i, j);
            }
        }
    }

    Ok(total_score.to_string())
}

fn bfs(map: &[Vec<u8>], start_i: usize, start_j: usize) -> usize {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut score = 0;

    queue.push_back((start_i, start_j, 0));
    visited[start_i][start_j] = true;

    while let Some((i, j, height)) = queue.pop_front() {
        if height == 9 {
            score += 1;
            continue;
        }

        for &(di, dj) in &directions {
            let ni = i as isize + di;
            let nj = j as isize + dj;

            if ni >= 0 && ni < map.len() as isize && nj >= 0 && nj < map[0].len() as isize {
                let ni = ni as usize;
                let nj = nj as usize;

                if !visited[ni][nj] && map[ni][nj] == height + 1 {
                    visited[ni][nj] = true;
                    queue.push_back((ni, nj, height + 1));
                }
            }
        }
    }

    score
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
        assert_eq!("36", process(input)?);
        Ok(())
    }
}
