use aoc2024_day_2::part2;

fn main() {
    tracing_subscriber::fmt::init();
    let input = include_str!("../../input2.txt");
    let result = part2::process(input);
    println!("Result: {}", result.expect("should have a result"));
}
