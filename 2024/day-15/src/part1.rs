use miette::*;
use nom::{
    character::complete::{char, line_ending},
    combinator::map,
    multi::{many0, many1, separated_list1},
    IResult, Parser,
};
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (_, (mut warehouse, moves)) =
        parse_input(input).map_err(|e| miette::miette!(e.to_string()))?;
    simulate_robot(&mut warehouse, &moves);
    let gps_sum = calculate_gps_sum(&warehouse);
    Ok(gps_sum.to_string())
}

#[derive(Debug)]
struct Warehouse {
    grid: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Moves {
    sequence: Vec<char>,
}

impl std::fmt::Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> IResult<&str, (Warehouse, Moves)> {
    let (input, grid) = parse_grid(input)?;
    let (input, _) = many0(line_ending)(input)?; // Handle optional newline
    let (input, moves) = parse_moves(input)?;
    Ok((input, (Warehouse { grid }, Moves { sequence: moves })))
}

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(
        line_ending,
        many1(
            char('#')
                .or(char('.'))
                .or(char('O'))
                .or(char('@'))
                .or(char(' ')),
        ),
    )(input)
}

fn parse_moves(input: &str) -> IResult<&str, Vec<char>> {
    map(
        separated_list1(
            line_ending,
            many1(char('^').or(char('v')).or(char('<')).or(char('>'))),
        ),
        |lines| lines.into_iter().flatten().collect(),
    )(input)
}

fn simulate_robot(warehouse: &mut Warehouse, moves: &Moves) {
    let (mut robot_x, mut robot_y) = find_robot(&warehouse.grid);
    let directions: HashMap<char, (i32, i32)> =
        vec![('^', (-1, 0)), ('v', (1, 0)), ('<', (0, -1)), ('>', (0, 1))]
            .into_iter()
            .collect();

    for &mv in &moves.sequence {
        if let Some(&(dx, dy)) = directions.get(&mv) {
            let mut new_x = (robot_x as i32 + dx) as usize;
            let mut new_y = (robot_y as i32 + dy) as usize;

            // Check if the robot can move or push boxes
            let mut can_move = true;
            let mut boxes_to_push = vec![];

            while warehouse.grid[new_x][new_y] == 'O' {
                boxes_to_push.push((new_x, new_y));
                new_x = (new_x as i32 + dx) as usize;
                new_y = (new_y as i32 + dy) as usize;
                if warehouse.grid[new_x][new_y] == '#' {
                    can_move = false;
                    break;
                }
            }

            // Check if the final position is a wall
            if warehouse.grid[new_x][new_y] == '#' {
                can_move = false;
            }

            if can_move {
                // Push the boxes
                let mut new_positions = vec![];
                for (box_x, box_y) in boxes_to_push {
                    let new_box_x = (box_x as i32 + dx) as usize;
                    let new_box_y = (box_y as i32 + dy) as usize;
                    new_positions.push((box_x, box_y, new_box_x, new_box_y));
                }

                for (box_x, box_y, _, _) in new_positions.iter().rev() {
                    warehouse.grid[*box_x][*box_y] = '.';
                }

                for (_, _, new_box_x, new_box_y) in &new_positions {
                    warehouse.grid[*new_box_x][*new_box_y] = 'O';
                }

                warehouse.grid[robot_x][robot_y] = '.';
                robot_x = (robot_x as i32 + dx) as usize;
                robot_y = (robot_y as i32 + dy) as usize;
                warehouse.grid[robot_x][robot_y] = '@';
            }
        }
    }
}

fn find_robot(grid: &[Vec<char>]) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '@') {
            return (i, j);
        }
    }
    panic!("Robot not found in the warehouse");
}

fn calculate_gps_sum(warehouse: &Warehouse) -> i32 {
    let mut sum = 0;
    for (i, row) in warehouse.grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'O' {
                sum += (i * 100 + j) as i32;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!("2028", process(input)?);
        Ok(())
    }
}
