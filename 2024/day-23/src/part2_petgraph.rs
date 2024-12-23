use std::collections::{HashMap, HashSet};

use miette::*;
use petgraph::{
    graph::{Graph, UnGraph},
    prelude::*,
};

#[cfg(debug_assertions)]
use std::{fs::File, io::Write};

#[cfg(debug_assertions)]
use petgraph::dot::{Config, Dot};

pub fn process(input: &str) -> Result<String> {
    let graph = build_network(input)?;
    #[cfg(debug_assertions)]
    plot_network(&graph, "network.dot")?;
    let largest_clique = find_largest_clique(&graph);
    Ok(largest_clique.iter().copied().collect::<Vec<_>>().join(","))
}

fn build_network(input: &str) -> Result<UnGraph<&str, ()>> {
    let line_count = input.lines().count();
    let mut graph = Graph::with_capacity(line_count, line_count * 2);
    let mut node_indices = HashMap::with_capacity(line_count * 2);

    for line in input.lines() {
        let (comp1, comp2) = line
            .split_once('-')
            .ok_or_else(|| miette!("Invalid line format"))?;

        let n1 = *node_indices
            .entry(comp1)
            .or_insert_with(|| graph.add_node(comp1));
        let n2 = *node_indices
            .entry(comp2)
            .or_insert_with(|| graph.add_node(comp2));
        graph.add_edge(n1, n2, ());
    }

    Ok(graph)
}

fn find_largest_clique<'a>(graph: &'a UnGraph<&'a str, ()>) -> HashSet<&'a str> {
    let mut largest_clique = HashSet::new();
    let nodes: Vec<_> = graph.node_indices().collect();

    for &start in &nodes {
        let mut current = vec![start];
        let mut candidates: Vec<_> = nodes
            .iter()
            .filter(|&&n| n > start && graph.contains_edge(start, n))
            .copied()
            .collect();

        while !candidates.is_empty() {
            let next = candidates[0];
            candidates.remove(0);

            current.push(next);
            if is_clique(graph, &current) {
                if current.len() > largest_clique.len() {
                    largest_clique = current.iter().map(|&n| graph[n]).collect();
                }
                candidates.retain(|&c| graph.contains_edge(next, c));
            } else {
                current.pop();
            }
        }
    }

    largest_clique
}

fn is_clique(graph: &UnGraph<&str, ()>, nodes: &[NodeIndex]) -> bool {
    nodes.iter().all(|&n1| {
        nodes
            .iter()
            .all(|&n2| n1 == n2 || graph.contains_edge(n1, n2))
    })
}

#[cfg(debug_assertions)]
fn plot_network(graph: &UnGraph<&str, ()>, filename: &str) -> Result<()> {
    let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    let mut file = File::create(filename).into_diagnostic()?;
    writeln!(file, "{:?}", dot).into_diagnostic()?;

    std::process::Command::new("sfdp")
        .args([
            "-Goverlap=scale",
            "-x",
            "-Tpdf",
            filename,
            "-o",
            "network.pdf",
        ])
        .output()
        .into_diagnostic()?;

    Ok(())
}
