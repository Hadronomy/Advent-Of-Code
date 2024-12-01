use {{crate_name}}::part2;

fn main() {
    tracing_subscriber::fmt::init();
    let input = include_str!("input.txt");
    let result = part2::process(input);
    println!("Result: {}", result.expect("should have a result"));
}
