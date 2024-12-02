use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

pub fn parse_line(line: &str) -> IResult<&str, Vec<usize>> {
    let (line, numbers) =
        separated_list1(tag(" "), map_res(digit1, |s: &str| s.parse::<usize>()))(line)?;
    Ok((line, numbers))
}

pub fn parse_reports(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(line_ending, parse_line)(input)
}
