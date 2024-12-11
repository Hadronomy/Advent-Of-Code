use indicatif::ProgressIterator;
use miette::*;
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let mut stone_counts: HashMap<Stone, u64> = input
        .split_whitespace()
        .map(|s| Stone {
            value: s.parse().unwrap(),
        })
        .fold(HashMap::new(), |mut acc, stone| {
            *acc.entry(stone).or_insert(0) += 1;
            acc
        });

    for _ in (0..75).progress() {
        let mut new_stone_counts = HashMap::new();
        for (stone, count) in stone_counts.iter() {
            let transformed = stone.transform();
            for new_stone in transformed {
                *new_stone_counts.entry(new_stone).or_insert(0) += count;
            }
        }

        stone_counts = new_stone_counts;
    }

    let total_stones: u64 = stone_counts.values().sum();
    Ok(total_stones.to_string())
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Stone {
    value: u64,
}

impl Stone {
    fn transform(&self) -> Vec<Stone> {
        match self.value {
            0 => vec![Stone { value: 1 }],
            v if (v as f64).log10() as usize % 2 == 1 => {
                let len = (v as f64).log10() as usize + 1;
                let mid = len / 2;
                let left = v / 10u64.pow(mid as u32);
                let right = v % 10u64.pow(mid as u32);
                vec![Stone { value: left }, Stone { value: right }]
            }
            _ => vec![Stone {
                value: self.value * 2024,
            }],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "125 17";
        assert_eq!("65601038650482", process(input)?);
        Ok(())
    }
}
