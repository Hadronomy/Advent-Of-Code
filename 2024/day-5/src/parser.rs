use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, map_res},
    multi::{many0, separated_list1},
    sequence::separated_pair,
    IResult,
};

use crate::safety_manual::{Page, Rule};

pub fn parse_input(input: &str) -> IResult<&str, (Vec<Rule>, Vec<Page>)> {
    let (input, rules) = parse_rules(input)?;
    let (input, _) = many0(newline)(input)?;
    let (input, updates) = parse_updates(input)?;
    Ok((input, (rules, updates)))
}

fn parse_page_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    map(
        separated_pair(parse_page_number, tag("|"), parse_page_number),
        |(a, b)| Rule::new(a, b),
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(newline, parse_rule)(input)
}

fn parse_update(input: &str) -> IResult<&str, Page> {
    map(separated_list1(tag(","), parse_page_number), |vec| {
        Page::new(vec)
    })(input)
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Page>> {
    separated_list1(newline, parse_update)(input)
}
