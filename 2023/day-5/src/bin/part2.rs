use aoc2023_day_5::part2;

fn main() {
    let input = include_str!("input.txt");
    let result = part2::process(input);
    println!("Result: {}", result.expect("should have a result"));
}
