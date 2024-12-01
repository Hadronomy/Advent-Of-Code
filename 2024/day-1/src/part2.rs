use std::collections::HashMap;
use std::vec;

use miette::*;
use rayon::prelude::*;
use indicatif::ParallelProgressIterator;

use crate::parser::*;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (_, data) = parse(input).expect("Failed to parse input");
    let vectors = data.into_iter().fold((vec![], vec![]), |mut acc, (a, b)| {
        acc.0.push(a);
        acc.1.push(b);
        acc
    });

    let number_occurences = vectors
        .1
        .par_iter()
        .progress()
        .fold(HashMap::new, |mut acc, b| {
            let counter = acc.entry(b).or_insert(0);
            *counter += 1;
            acc
        })
        .reduce(HashMap::new, |mut acc, map| {
            for (key, value) in map {
                *acc.entry(key).or_insert(0) += value;
            }
            acc
        });

    let result = vectors
        .0
        .par_iter()
        .progress()
        .zip(vectors.1.par_iter())
        .map(|(a, _)| a * number_occurences.get(a).unwrap_or(&0))
        .sum::<u32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
