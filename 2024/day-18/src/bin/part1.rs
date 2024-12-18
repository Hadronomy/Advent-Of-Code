use miette::*;

use aoc2024_day_18::part1;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let input = include_str!("../../input1.txt");
    let result = part1::process(input)?;
    println!("Result: {}", result);
    Ok(())
}
