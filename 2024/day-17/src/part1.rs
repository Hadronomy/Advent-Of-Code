use miette::*;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map_res,
    multi::{many0, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (register_a, _, _, _) = parse_input(input)?;

    let mut a = register_a;
    let mut ans = Vec::new();
    while a > 0 {
        ans.push(hardcoded_program(a).to_string());
        a >>= 3;
    }
    Ok(ans.join(","))
}

/// 2,4,1,7,7,5,1,7,4,6,0,3,5,5,3,0
/// 0: b = a % 8
/// 2: b ^= 7
/// 4: c = a >> b
/// 6: b ^= 7
/// 8: b ^= c
/// 10: a >>= 3
/// 12: out b % 8
/// jnz 0
fn hardcoded_program(a: usize) -> u8 {
    ((a ^ (a >> ((a % 8) ^ 7)) ^ 8) % 8) as u8
}

fn parse_input(input: &str) -> Result<(usize, usize, usize, Vec<u8>)> {
    let (_, (register_a, register_b, register_c, program)) =
        parse_all(input).map_err(|e| miette!("Failed to parse input {e}"))?;
    Ok((register_a, register_b, register_c, program))
}

fn parse_all(input: &str) -> IResult<&str, (usize, usize, usize, Vec<u8>)> {
    let (input, register_a) = parse_register("Register A")(input)?;
    let (input, register_b) = parse_register("Register B")(input)?;
    let (input, register_c) = parse_register("Register C")(input)?;
    let (input, program) = preceded(tag("Program: "), parse_program)(input)?;
    Ok((input, (register_a, register_b, register_c, program)))
}

fn parse_register<'a>(label: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, usize> {
    move |input: &'a str| {
        let (input, (_, value)) =
            separated_pair(tag(label), tag(": "), map_res(digit1, str::parse))(input)?;
        let (input, _) = many0(newline)(input)?;
        Ok((input, value))
    }
}

fn parse_program(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(tag(","), map_res(digit1, str::parse))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", process(input)?);
        Ok(())
    }
}
