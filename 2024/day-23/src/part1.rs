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

    let mut triplet_count = 0;
    let computers: Vec<&str> = network.keys().copied().collect();

    for i in 0..computers.len() {
        for j in (i + 1)..computers.len() {
            for k in (j + 1)..computers.len() {
                let comp1 = computers[i];
                let comp2 = computers[j];
                let comp3 = computers[k];

                if network[comp1].contains(comp2)
                    && network[comp1].contains(comp3)
                    && network[comp2].contains(comp3)
                    && (comp1.starts_with('t') || comp2.starts_with('t') || comp3.starts_with('t'))
                {
                    triplet_count += 1;
                }
            }
        }
    }

    Ok(triplet_count.to_string())
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
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
