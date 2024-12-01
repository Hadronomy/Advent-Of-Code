use nom::character::complete::{digit1, line_ending};
use nom::IResult;

pub fn parse_line(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, a) = digit1(input)?;
    let (input, _) = nom::character::complete::space1(input)?;
    let (input, b) = digit1(input)?;
    let (input, _) = nom::combinator::opt(line_ending)(input)?;
    Ok((input, (a.parse().unwrap(), b.parse().unwrap())))
}

pub fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    nom::multi::many1(parse_line)(input)
}
