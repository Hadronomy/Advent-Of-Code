use miette::*;

fn transform_secret(mut secret: i64) -> i64 {
    let result = secret * 64;
    secret ^= result;
    secret %= 16777216;

    let result = secret / 32;
    secret ^= result;
    secret %= 16777216;

    let result = secret * 2048;
    secret ^= result;
    secret %= 16777216;

    secret
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let initial_secrets: Vec<i64> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i64>())
        .collect::<std::result::Result<_, _>>()
        .map_err(|e| miette!("Failed to parse input: {}", e))?;

    let sum: i64 = initial_secrets
        .iter()
        .map(|&secret| {
            let mut current = secret;
            for _ in 0..2000 {
                current = transform_secret(current);
            }
            current
        })
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "1
2
3
2024";
        assert_eq!("23", process(input)?);
        Ok(())
    }
}
