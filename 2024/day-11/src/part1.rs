use miette::*;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let mut stones: Vec<Stone> = input
        .split_whitespace()
        .map(|s| Stone {
            value: s.parse().unwrap(),
        })
        .collect();

    for _ in 0..25 {
        let mut new_stones = Vec::new();
        for stone in &stones {
            let transformed = stone.transform();
            new_stones.extend(transformed);
        }

        stones = new_stones;
    }

    Ok(stones.len().to_string())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Stone {
    value: u64,
}

impl Stone {
    fn transform(&self) -> Vec<Stone> {
        match self.value {
            0 => vec![Stone { value: 1 }],
            v if v.to_string().len() % 2 == 0 => {
                let s = v.to_string();
                let mid = s.len() / 2;
                let left = s[..mid].parse().unwrap_or(0);
                let right = s[mid..].parse().unwrap_or(0);
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
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
