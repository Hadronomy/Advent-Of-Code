use miette::*;
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let mut sections = input.split("\n\n");
    let patterns: Vec<&str> = sections.next().unwrap().split(", ").collect();
    let designs: Vec<&str> = sections.next().unwrap().lines().collect();

    let mut memo = HashMap::new();
    let mut total_ways = 0;

    for design in designs {
        total_ways += count_ways(design, &patterns, &mut memo);
    }

    Ok(total_ways.to_string())
}

fn count_ways(design: &str, patterns: &[&str], memo: &mut HashMap<String, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&result) = memo.get(design) {
        return result;
    }
    let mut ways = 0;
    for &pattern in patterns {
        if design.starts_with(pattern) {
            let remaining = design.strip_prefix(pattern).unwrap();
            ways += count_ways(remaining, patterns, memo);
        }
    }
    memo.insert(design.to_string(), ways);
    ways
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
        assert_eq!("16", process(input)?);
        Ok(())
    }
}
