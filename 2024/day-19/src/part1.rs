use miette::*;
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let mut sections = input.split("\n\n");
    let patterns: Vec<&str> = sections.next().unwrap().split(", ").collect();
    let designs: Vec<&str> = sections.next().unwrap().lines().collect();

    let mut memo = HashMap::new();
    let mut count = 0;

    for design in designs {
        if can_construct(design, &patterns, &mut memo) {
            count += 1;
        }
    }

    Ok(count.to_string())
}

fn can_construct(design: &str, patterns: &[&str], memo: &mut HashMap<String, bool>) -> bool {
    if design.is_empty() {
        return true;
    }
    if let Some(&result) = memo.get(design) {
        return result;
    }
    for &pattern in patterns {
        if design.starts_with(pattern) {
            let remaining = design.strip_prefix(pattern).unwrap();
            if can_construct(remaining, patterns, memo) {
                memo.insert(design.to_string(), true);
                return true;
            }
        }
    }
    memo.insert(design.to_string(), false);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
