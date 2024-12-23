use miette::*;

use aoc2024_day_23::part2_petgraph;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let input = include_str!("../../input2.txt");
    let result = part2_petgraph::process(input)?;
    println!("Result: {}", result);
    Ok(())
}
