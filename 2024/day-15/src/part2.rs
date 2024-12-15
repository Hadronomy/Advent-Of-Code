use miette::*;
use nom::{
    character::complete::{char, line_ending},
    combinator::map,
    multi::{many0, many1, separated_list1},
    IResult, Parser,
};
use std::collections::{HashMap, HashSet};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (_, (warehouse, moves)) = parse_input(input).map_err(|e| miette::miette!(e.to_string()))?;
    let mut scaled_warehouse = warehouse.scale_up();
    simulate_robot(&mut scaled_warehouse, &moves);
    let gps_sum = calculate_gps_sum(&scaled_warehouse);
    Ok(gps_sum.to_string())
}

#[derive(Debug)]
struct Warehouse {
    grid: Vec<Vec<char>>,
}

impl Warehouse {
    fn scale_up(&self) -> Warehouse {
        let mut new_grid = Vec::new();
        for row in &self.grid {
            let mut new_row = Vec::new();
            for &tile in row {
                match tile {
                    '#' => new_row.extend_from_slice(&['#', '#']),
                    'O' => new_row.extend_from_slice(&['[', ']']),
                    '.' => new_row.extend_from_slice(&['.', '.']),
                    '@' => new_row.extend_from_slice(&['@', '.']),
                    _ => new_row.push(tile),
                }
            }
            new_grid.push(new_row);
        }
        Warehouse { grid: new_grid }
    }
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
    warehouse.grid[robot_x][robot_y] = '.'; // Remove robot initial position for consistent movement logic

    let directions: HashMap<char, (i32, i32)> =
        vec![('^', (-1, 0)), ('v', (1, 0)), ('<', (0, -1)), ('>', (0, 1))]
            .into_iter()
            .collect();

    for &mv in &moves.sequence {
        if let Some(&(dx, dy)) = directions.get(&mv) {
            let next_pos = (robot_x as i32 + dx, robot_y as i32 + dy);

            if !is_within_bounds(&warehouse.grid, next_pos)
                || warehouse.grid[next_pos.0 as usize][next_pos.1 as usize] == '#'
            {
                continue;
            }

            match warehouse.grid[next_pos.0 as usize][next_pos.1 as usize] {
                '.' => {
                    robot_x = next_pos.0 as usize;
                    robot_y = next_pos.1 as usize;
                }
                '[' | ']' => {
                    let to_move = bfs_reach(next_pos, |pos| {
                        let mut results = vec![];
                        let cell = warehouse.grid[pos.0 as usize][pos.1 as usize];

                        if cell == '[' {
                            results.push((pos.0, pos.1 + 1)); // Extend right
                        }
                        if cell == ']' {
                            results.push((pos.0, pos.1 - 1)); // Extend left
                        }
                        for &(dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                            let adj = (pos.0 + dx, pos.1 + dy);
                            if is_within_bounds(&warehouse.grid, adj)
                                && warehouse.grid[adj.0 as usize][adj.1 as usize] == cell
                            {
                                results.push(adj);
                            }
                        }
                        results
                    });

                    let mut can_move = true;
                    let mut updated_positions = vec![];

                    for &(box_x, box_y) in &to_move {
                        let new_box_pos = (box_x + dx, box_y + dy);
                        if !is_within_bounds(&warehouse.grid, new_box_pos)
                            || warehouse.grid[new_box_pos.0 as usize][new_box_pos.1 as usize] == '#'
                        {
                            can_move = false;
                            break;
                        }
                        updated_positions.push(((box_x, box_y), new_box_pos));
                    }

                    if can_move {
                        for &((old_x, old_y), (new_x, new_y)) in &updated_positions {
                            warehouse.grid[old_x as usize][old_y as usize] = '.';
                            warehouse.grid[new_x as usize][new_y as usize] = '[';
                        }

                        robot_x = next_pos.0 as usize;
                        robot_y = next_pos.1 as usize;
                    }
                }
                _ => {}
            }
        }
        println!("{}", warehouse);
    }

    warehouse.grid[robot_x][robot_y] = '@';
}

fn calculate_gps_sum(warehouse: &Warehouse) -> usize {
    let mut gps_sum = 0;
    for (y, row) in warehouse.grid.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if tile == '[' {
                gps_sum += 100 * y + x;
            }
        }
    }
    gps_sum
}

fn find_robot(grid: &[Vec<char>]) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '@') {
            return (i, j);
        }
    }
    panic!("Robot not found in the warehouse");
}

fn is_within_bounds(grid: &[Vec<char>], pos: (i32, i32)) -> bool {
    let (x, y) = pos;
    x >= 0 && y >= 0 && (x as usize) < grid.len() && (y as usize) < grid[0].len()
}

fn bfs_reach<F>(start: (i32, i32), neighbors: F) -> Vec<(i32, i32)>
where
    F: Fn((i32, i32)) -> Vec<(i32, i32)>,
{
    let mut visited = HashSet::new();
    let mut to_visit = vec![start];
    let mut result = vec![];

    while let Some(pos) = to_visit.pop() {
        if visited.insert(pos) {
            result.push(pos);
            to_visit.extend(neighbors(pos));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        assert_eq!("9021", process(input)?);
        Ok(())
    }
}
