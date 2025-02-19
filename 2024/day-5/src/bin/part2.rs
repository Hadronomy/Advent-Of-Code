use miette::*;

use aoc2024_day_5::part2;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let input = include_str!("../../input2.txt");
    let result = part2::process(input)?;
    println!("Result: {}", result);
    Ok(())
}
