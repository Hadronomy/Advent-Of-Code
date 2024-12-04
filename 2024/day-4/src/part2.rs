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
    let (_, grid) = parse_input(input).map_err(|_| miette::miette!("Failed to parse input"))?;
    let count = count_x_mas(&grid);
    Ok(count.to_string())
}

type Span<'a> = LocatedSpan<&'a str>;

fn parse_input(input: &str) -> IResult<Span, Vec<Vec<char>>> {
    let parse_line = map(alpha1, |s: Span| s.chars().collect::<Vec<_>>());
    let parse_grid = separated_list1(line_ending, parse_line);
    let (remaining, grid) = terminated(parse_grid, opt(line_ending))(Span::new(input))?;
    Ok((remaining, grid))
}

fn count_x_mas(grid: &[Vec<char>]) -> usize {
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    // The patterns are encoded
    // from the top-left corner of the 3x3 grid
    // to the bottom-right corner as follows:
    // [top-left, top-right, bottom-left, bottom-right]
    let patterns = [
        ['M', 'S', 'M', 'S'],
        ['S', 'M', 'S', 'M'],
        ['S', 'S', 'M', 'M'],
        ['M', 'M', 'S', 'S'],
    ];

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if grid[row][col] == 'A' {
                for pattern in &patterns {
                    if grid[row - 1][col - 1] == pattern[0]
                        && grid[row - 1][col + 1] == pattern[1]
                        && grid[row + 1][col - 1] == pattern[2]
                        && grid[row + 1][col + 1] == pattern[3]
                    {
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
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
