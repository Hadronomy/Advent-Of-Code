use itertools::Itertools;
use miette::*;
use std::collections::{HashMap, VecDeque};

type Grid = Vec<Vec<Option<char>>>;
type Sequences = HashMap<(char, char), Vec<String>>;

pub fn process(input: &str) -> Result<String> {
    let num_keypad = vec![
        vec![Some('7'), Some('8'), Some('9')],
        vec![Some('4'), Some('5'), Some('6')],
        vec![Some('1'), Some('2'), Some('3')],
        vec![None, Some('0'), Some('A')],
    ];

    let dir_keypad = vec![
        vec![None, Some('^'), Some('A')],
        vec![Some('<'), Some('v'), Some('>')],
    ];

    let num_sequences = compute_sequences(&num_keypad);
    let dir_sequences = compute_sequences(&dir_keypad);
    let dir_lengths: HashMap<(char, char), usize> = dir_sequences
        .iter()
        .map(|(&k, v)| (k, v[0].len()))
        .collect();

    let mut cache = HashMap::new();
    let total: u64 = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let numeric = line[..line.len() - 1].parse::<u64>().unwrap_or(0);
            let inputs = solve(line, &num_sequences);
            let length = inputs
                .iter()
                .map(|seq| compute_length(seq, 25, &dir_sequences, &dir_lengths, &mut cache))
                .min()
                .unwrap_or(0);
            numeric * length
        })
        .sum();

    Ok(total.to_string())
}

fn compute_sequences(keypad: &Grid) -> Sequences {
    let mut position = HashMap::new();
    for (r, row) in keypad.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if let Some(ch) = cell {
                position.insert(*ch, (r, c));
            }
        }
    }

    let mut sequences = HashMap::new();
    for &x in position.keys() {
        for &y in position.keys() {
            if x == y {
                sequences.insert((x, y), vec!["A".to_string()]);
                continue;
            }

            let mut possibilities = Vec::new();
            let mut queue = VecDeque::new();
            queue.push_back((position[&x], String::new()));
            let mut optimal = i32::MAX;

            'outer: while let Some(((row, col), moves)) = queue.pop_front() {
                for (nr, nc, nm) in [
                    (row.wrapping_sub(1), col, '^'),
                    (row + 1, col, 'v'),
                    (row, col.wrapping_sub(1), '<'),
                    (row, col + 1, '>'),
                ] {
                    if nr >= keypad.len() || nc >= keypad[0].len() {
                        continue;
                    }
                    if keypad[nr][nc].is_none() {
                        continue;
                    }

                    if keypad[nr][nc] == Some(y) {
                        if optimal < (moves.len() + 1) as i32 {
                            break 'outer;
                        }
                        optimal = (moves.len() + 1) as i32;
                        let mut path = moves.clone();
                        path.push(nm);
                        path.push('A');
                        possibilities.push(path);
                    } else {
                        let mut new_moves = moves.clone();
                        new_moves.push(nm);
                        queue.push_back(((nr, nc), new_moves));
                    }
                }
            }
            sequences.insert((x, y), possibilities);
        }
    }
    sequences
}

fn solve(string: &str, sequences: &Sequences) -> Vec<String> {
    let pairs: Vec<_> = std::iter::once('A')
        .chain(string.chars())
        .tuple_windows()
        .collect();

    let options: Vec<_> = pairs.iter().map(|(x, y)| &sequences[&(*x, *y)]).collect();

    options
        .iter()
        .map(|v| v.iter())
        .multi_cartesian_product()
        .map(|v| v.iter().flat_map(|s| s.chars()).collect::<String>())
        .collect()
}

fn compute_length(
    sequence: &str,
    depth: i32,
    dir_sequences: &Sequences,
    dir_lengths: &HashMap<(char, char), usize>,
    cache: &mut HashMap<(String, i32), u64>,
) -> u64 {
    // Changed return type to u64
    if let Some(&cached) = cache.get(&(sequence.to_string(), depth)) {
        return cached;
    }

    if depth == 1 {
        let length: u64 = std::iter::once('A')
            .chain(sequence.chars())
            .tuple_windows()
            .map(|(x, y)| dir_lengths[&(x, y)] as u64)
            .sum();
        cache.insert((sequence.to_string(), depth), length);
        return length;
    }

    let length: u64 = std::iter::once('A')
        .chain(sequence.chars())
        .tuple_windows()
        .map(|(x, y)| {
            dir_sequences[&(x, y)]
                .iter()
                .map(|subseq| compute_length(subseq, depth - 1, dir_sequences, dir_lengths, cache))
                .min()
                .unwrap_or(0)
        })
        .sum();

    cache.insert((sequence.to_string(), depth), length);
    length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "029A\n980A\n179A\n456A\n379A";
        assert_eq!("154115708116294", process(input)?);
        Ok(())
    }
}
