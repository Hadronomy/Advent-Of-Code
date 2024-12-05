use miette::*;

use crate::{parser::parse_input, safety_manual::ProductionInstructions};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (_, (rules, updates)) = parse_input(input).map_err(|_| miette!("Failed to parse input"))?;
    let safety_manual = ProductionInstructions::new(rules, updates);
    let sum = safety_manual.sum_of_middle_page_numbers();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[test]
    fn it_works() -> Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
