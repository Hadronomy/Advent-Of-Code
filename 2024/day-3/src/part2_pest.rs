use miette::*;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "part2.pest"]
struct Part2Parser;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let multiplications = parse(input)?;
    let result = multiplications.iter().fold(0, |acc, (a, b)| acc + a * b);
    Ok(result.to_string())
}

type Multiplication = (u32, u32);

fn parse(input: &str) -> Result<Vec<Multiplication>> {
    let mut multiplications = Vec::new();
    let mut enabled = true;

    let pairs = Part2Parser::parse(Rule::input, input).into_diagnostic()?;
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::do_instruction => enabled = true,
                Rule::dont_instruction => enabled = false,
                Rule::mul_instruction if enabled => {
                    let mut inner_pairs = inner_pair.into_inner();
                    let a = inner_pairs
                        .next()
                        .unwrap()
                        .as_str()
                        .parse()
                        .into_diagnostic()?;
                    let b = inner_pairs
                        .next()
                        .unwrap()
                        .as_str()
                        .parse()
                        .into_diagnostic()?;
                    multiplications.push((a, b));
                }
                _ => {}
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
