use {{crate_name}}::part1;

fn main() {
    tracing_subscriber::fmt::init();
    let input = include_str!("../../input1.txt");
    let result = part1::process(input);
    println!("Result: {}", result.expect("should have a result"));
}
