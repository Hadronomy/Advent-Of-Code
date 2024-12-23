use std::collections::{HashMap, HashSet};

use miette::*;

fn transform_secret(mut secret: i64) -> i64 {
    let modulo_operand = 16777216;
    let mut result = secret * 64;
    secret ^= result;
    secret %= modulo_operand;

    result = secret / 32;
    secret ^= result;
    secret %= modulo_operand;

    result = secret * 2048;
    secret ^= result;
    secret %= modulo_operand;

    secret
}

fn get_price(secret: i64) -> i64 {
    secret.abs() % 10
}

fn get_buyer_prices(initial: i64) -> Vec<i64> {
    let mut prices = Vec::with_capacity(2001);
    let mut secret = initial;
    prices.push(get_price(secret));

    for _ in 0..2000 {
        secret = transform_secret(secret);
        prices.push(get_price(secret));
    }
    prices
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let initial_secrets: Vec<i64> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i64>())
        .collect::<std::result::Result<_, _>>()
        .map_err(|e| miette!("Failed to parse input: {}", e))?;

    let mut sequence_totals: HashMap<[i64; 4], i64> = HashMap::new();

    for &initial in &initial_secrets {
        let prices = get_buyer_prices(initial);
        let mut seen_sequences = HashSet::new();

        for window in prices.windows(5) {
            let sequence = [
                window[1] - window[0],
                window[2] - window[1],
                window[3] - window[2],
                window[4] - window[3],
            ];

            if seen_sequences.insert(sequence) {
                *sequence_totals.entry(sequence).or_default() += window[4];
            }
        }
    }

    Ok(sequence_totals.values().max().unwrap_or(&0).to_string())
}
