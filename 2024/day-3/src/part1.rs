use miette::*;
use nom::{bytes::complete::tag, character::complete::digit1, IResult};
use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let multiplications = parse(input)?;
    let result = multiplications.iter().fold(0, |acc, (a, b)| acc + a * b);
    Ok(result.to_string())
}

type Multiplication = (u32, u32);

fn parse_multiplication(input: &str) -> IResult<&str, Multiplication> {
    let (input, _) = tag("mul(")(input)?;
    let (input, a) = digit1(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, b) = digit1(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, (a.parse().unwrap(), b.parse().unwrap())))
}

fn parse(input: &str) -> Result<Vec<Multiplication>> {
    let regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut multiplications = Vec::new();

    for r#match in regex.find_iter(input) {
        let substring = r#match.as_str();
        let (_, multiplication) = parse_multiplication(substring)
            .map_err(|_| miette::Error::msg("Failed to parse multiplication"))?;
        multiplications.push(multiplication);
    }

    Ok(multiplications)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = process(input).unwrap();
        assert_eq!(result, "161");
    }
}
