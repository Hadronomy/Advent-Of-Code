use miette::*;
use nom::{bytes::complete::tag, character::complete::digit1, IResult};
use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let multiplications = parse(input)?;
    let result = multiplications.iter().fold(0, |acc, (a, b)| acc + a * b);
    Ok(result.to_string())
}

enum Instruction {
    Do,
    Dont,
    Multiplication(Multiplication),
}

type Multiplication = (u32, u32);

fn parse_multiplication(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul(")(input)?;
    let (input, a) = digit1(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, b) = digit1(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((
        input,
        Instruction::Multiplication((a.parse().unwrap(), b.parse().unwrap())),
    ))
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Instruction::Do))
}

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Instruction::Dont))
}

fn parse(input: &str) -> Result<Vec<Multiplication>> {
    let regex = Regex::new(r"(do\(\)|don't\(\)|mul\(\d+,\d+\))").unwrap();
    let mut multiplications = Vec::new();
    let mut enabled = true;

    for r#match in regex.find_iter(input) {
        let substring = r#match.as_str();
        if let Ok((_, _)) = parse_do(substring) {
            enabled = true;
        } else if let Ok((_, _)) = parse_dont(substring) {
            enabled = false;
        } else if enabled {
            let (_, multiplication) = parse_multiplication(substring)
                .map_err(|_| miette::Error::msg("Failed to parse multiplication"))?;
            if let Instruction::Multiplication(mul) = multiplication {
                multiplications.push(mul);
            }
        }
    }

    Ok(multiplications)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = process(input).unwrap();
        assert_eq!(result, "48");
    }
}
