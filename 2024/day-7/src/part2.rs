use miette::*;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use nom_locate::LocatedSpan;
use std::str::FromStr;
use strum::{EnumIter, IntoEnumIterator};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let equations = parse(input)?;
    let total: usize = equations
        .into_iter()
        .filter(|eq| evaluate(&eq.numbers, eq.target))
        .map(|eq| eq.target)
        .sum();

    Ok(total.to_string())
}

fn parse(input: &str) -> Result<Vec<Equation>> {
    let input = LocatedSpan::new(input);
    let (_, equations) =
        separated_list1(newline, parse_equation)(input).map_err(|e| miette!(e.to_string()))?;
    Ok(equations)
}

fn parse_equation(input: LocatedSpan<&str>) -> IResult<LocatedSpan<&str>, Equation> {
    let (input, (target, numbers)) = separated_pair(
        map_res(digit1, |s: LocatedSpan<&str>| usize::from_str(*s)),
        tag(": "),
        separated_list1(
            space1,
            map_res(digit1, |s: LocatedSpan<&str>| usize::from_str(*s)),
        ),
    )(input)?;

    Ok((input, Equation { target, numbers }))
}

fn evaluate(numbers: &[usize], target: usize) -> bool {
    fn helper(numbers: &[usize], current: usize, target: usize) -> bool {
        if numbers.is_empty() {
            return current == target;
        }

        let next = numbers[0];
        let rest = &numbers[1..];

        Operation::iter().any(|operation| helper(rest, operation.apply(current, next), target))
    }

    if numbers.is_empty() {
        return false;
    }

    helper(&numbers[1..], numbers[0], target)
}

#[derive(Debug)]
struct Equation {
    target: usize,
    numbers: Vec<usize>,
}

#[derive(Debug, EnumIter)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl Operation {
    fn apply(&self, current: usize, next: usize) -> usize {
        match self {
            Operation::Add => current + next,
            Operation::Multiply => current * next,
            Operation::Concatenate => format!("{}{}", current, next).parse::<usize>().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("11387", process(input)?);
        Ok(())
    }
}
