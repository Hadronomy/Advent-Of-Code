use miette::*;
use nom::{
    character::complete::{alpha1, line_ending},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
use nom_locate::LocatedSpan;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let grid = parse_input(input)
        .map_err(|_| miette!("Failed to parse input"))?
        .1;
    let count = count_xmas(&grid);
    Ok(count.to_string())
}

type Span<'a> = LocatedSpan<&'a str>;

fn parse_input(input: &str) -> IResult<Span, Vec<Vec<char>>> {
    let parse_line = map(alpha1, |s: Span| s.chars().collect::<Vec<_>>());
    let parse_grid = separated_list1(line_ending, parse_line);
    let (remaining, grid) = terminated(parse_grid, opt(line_ending))(Span::new(input))?;
    Ok((remaining, grid))
}

fn count_xmas(grid: &[Vec<char>]) -> usize {
    let directions = vec![
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // down-right
        (1, -1),  // down-left
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // up-left
        (-1, 1),  // up-right
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let complete_word = "XMAS";
    let start = complete_word.chars().next().unwrap();
    let word = &complete_word[1..];

    fn check_direction(
        grid: &[Vec<char>],
        word: &str,
        start_row: isize,
        start_col: isize,
        dir: (isize, isize),
    ) -> bool {
        for (i, ch) in word.chars().enumerate() {
            let new_row = start_row + (i + 1) as isize * dir.0;
            let new_col = start_col + (i + 1) as isize * dir.1;
            if new_row < 0
                || new_row >= grid.len() as isize
                || new_col < 0
                || new_col >= grid[0].len() as isize
            {
                return false;
            }
            if grid[new_row as usize][new_col as usize] != ch {
                return false;
            }
        }
        true
    }

    let mut count = 0;
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == start {
                for dir in &directions {
                    if check_direction(grid, word, row as isize, col as isize, *dir) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
