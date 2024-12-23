use itertools::Itertools;
use miette::*;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "part2.pest"]
struct Part2Parser;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let multiplications = parse(input)?;
    let result: u32 = multiplications.iter().map(|(a, b)| a * b).sum();
    Ok(result.to_string())
}

type Multiplication = (u32, u32);

fn parse(input: &str) -> Result<Vec<Multiplication>> {
    let pairs = Part2Parser::parse(Rule::input, input).into_diagnostic()?;

    let mut multiplications = Vec::with_capacity(pairs.clone().count());
    let mut enabled = true;

    for pair in pairs {
        match pair.as_rule() {
            Rule::do_instruction => enabled = true,
            Rule::dont_instruction => enabled = false,
            Rule::mul_instruction if enabled => {
                let (a, b) = pair
                    .into_inner()
                    .map(|p| p.as_str().parse::<u32>().into_diagnostic())
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .take(2)
                    .collect_tuple()
                    .ok_or_else(|| miette!("Invalid multiplication instruction"))?;
                multiplications.push((a, b));
            }
            _ => continue,
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
