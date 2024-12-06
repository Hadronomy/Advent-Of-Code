use indicatif::ProgressBar;
use miette::*;
use std::collections::HashSet;

pub fn process(input: &str) -> Result<String> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut position = (0, 0);
    let mut direction = Direction::Up;

    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '^' {
                position = (x, y);
                direction = Direction::Up;
                break;
            }
        }
    }

    let pb = ProgressBar::new_spinner();
    loop {
        visited.insert(position);
        let next_position = direction.move_forward(position);

        if next_position.1 >= map.len() || next_position.0 >= map[0].len() {
            break;
        }

        if map[next_position.1][next_position.0] == '#' {
            direction = direction.turn_right();
        } else {
            position = next_position;
        }

        pb.inc(1);
    }
    pb.finish_with_message("Simulation complete");

    Ok(visited.len().to_string())
}

#[derive(Clone, Copy, Debug)]
enum Direction {
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

    fn move_forward(self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (x, y.saturating_sub(1)),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x.saturating_sub(1), y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[test]
    fn it_works() -> Result<()> {
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
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
