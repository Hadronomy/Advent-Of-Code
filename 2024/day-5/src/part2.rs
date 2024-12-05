use miette::*;

use crate::{parser::parse_input, safety_manual::Page};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (_, (rules, pages)) = parse_input(input).map_err(|_| miette!("Failed to parse input"))?;

    let incorrect_updates: Vec<Page> = pages
        .into_iter()
        .filter(|page| !page.is_correctly_ordered(&rules))
        .collect();

    let corrected_sum: u32 = incorrect_updates
        .into_iter()
        .map(|page| page.correct_order(&rules).middle_page_number())
        .sum();

    Ok(corrected_sum.to_string())
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
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
