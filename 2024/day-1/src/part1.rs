use std::vec;

use miette::*;
use rayon::prelude::*;

use crate::parser::*;

/// Calculate the distance between two numbers
/// ```
/// let a: u32 = 3;
/// let b: u32 = 4;
/// let result = distance(a, b);
/// ```
fn distance(a: &u32, b: &u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (_, data) = parse(input).expect("Failed to parse input");
    let mut vectors = data.into_iter().fold((vec![], vec![]), |mut acc, (a, b)| {
        acc.0.push(a);
        acc.1.push(b);
        acc
    });

    vectors.0.sort();
    vectors.1.sort();

    let result = vectors
        .0
        .par_iter()
        .zip(vectors.1.par_iter())
        .map(|(a, b)| distance(a, b))
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
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
