use miette::*;
use std::collections::{HashMap, HashSet};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let mut network: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let (comp1, comp2) = line.split_once('-').unwrap();
        network.entry(comp1).or_default().insert(comp2);
        network.entry(comp2).or_default().insert(comp1);
    }

    let computers: Vec<&str> = network.keys().copied().collect();
    let mut largest_clique: Vec<&str> = Vec::new();

    for &start in &computers {
        let mut current = vec![start];
        let mut candidates: Vec<&str> = computers
            .iter()
            .filter(|&&c| c > start && network[start].contains(c))
            .copied()
            .collect();

        while !candidates.is_empty() {
            let next = candidates[0];
            candidates.remove(0);

            current.push(next);
            if is_clique(&current, &network) {
                if current.len() > largest_clique.len() {
                    largest_clique = current.clone();
                }

                candidates.retain(|&c| network[next].contains(c));
            } else {
                current.pop();
            }
        }
    }

    largest_clique.sort();
    Ok(largest_clique.join(","))
}

/// Finds the largest fully connected subgraph containing the given computers
fn is_clique(computers: &[&str], network: &HashMap<&str, HashSet<&str>>) -> bool {
    for i in 0..computers.len() {
        for j in (i + 1)..computers.len() {
            if !network[computers[i]].contains(computers[j]) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!("co,de,ka,ta", process(input)?);
        Ok(())
    }
}
