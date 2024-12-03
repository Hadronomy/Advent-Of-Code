use miette::*;

use {{crate_name}}::part1;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let input = include_str!("../../input1.txt");
    let result = part1::process(input)?;
    println!("Result: {}", result);
    Ok(())
}