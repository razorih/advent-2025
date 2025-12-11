use std::{collections::{HashMap, VecDeque}, io};

use advent_2025::read_input_from_env;

fn silver(graph: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut paths = 0;

    // do a simple BFS search without breaking
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back("you");

    while let Some(current) = queue.pop_front() {
        if current == "out" {
            paths += 1;
        }

        if let Some(child_nodes) = graph.get(current) {
            queue.extend(child_nodes);
        }
    }

    paths
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let (ins, outs) = line.split_once(':')?;
            let outs = outs.trim().split_ascii_whitespace().collect();

            Some((ins, outs))
        })
        .collect()
}

/// Visualize graph in Graphviz DOT language
#[allow(dead_code)]
fn to_graphviz(graph: &HashMap<&str, Vec<&str>>) -> String {
    let mut out = String::new();

    out.push_str("digraph G {\n");

    for (&in_node, outs) in graph {
        for out_node in outs {
            out.push_str(&format!("{in_node} -> {out_node};\n"));
        }
    }

    out.push_str("you [shape=diamond, style=filled, fillcolor=cyan];\n");
    out.push_str("out [shape=dsquare, style=filled, fillcolor=red];\n");
    out.push_str("}");

    out
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    let graph = parse(&input);

    // println!("{}", to_graphviz(&graph));
    println!("silver: {}", silver(&graph));

    Ok(())
}
