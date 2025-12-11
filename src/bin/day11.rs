use std::{collections::HashMap, io};

use advent_2025::read_input_from_env;

fn silver(graph: &HashMap<&str, Vec<&str>>) -> u64 {
    count_paths(graph, "you", "out")
}

fn gold(graph: &HashMap<&str, Vec<&str>>) -> u64 {
    // since we're passing through fft and dac
    // we can multiply unique paths
    //   svr -> fft
    //   fft -> dac
    //   dac -> out
    let svr_to_fft = count_paths(graph, "svr", "fft");
    let fft_to_dac = count_paths(graph, "fft", "dac");
    let dac_to_out = count_paths(graph, "dac", "out");

    // in reality there's only fft -> dac path, but verify
    debug_assert_eq!(count_paths(graph, "dac", "fft"), 0);

    svr_to_fft * fft_to_dac * dac_to_out
}

// https://stackoverflow.com/a/79295398
fn recurse_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<&'a str, u64>,
    from: &'a str,
    to: &'a str,
) -> u64 {
    // base case
    if from == to {
        return 1;
    }

    // return if we already know how many paths exists to target from this node
    if let Some(remembered) = memo.get(from) {
        return *remembered;
    }

    let mut path_count = 0;
    for child in graph.get(from).unwrap_or(&vec![]) {
        path_count += recurse_paths(graph, memo, child, to);
    }

    memo.insert(from, path_count);
    path_count
}

fn count_paths(graph: &HashMap<&str, Vec<&str>>, from: &str, to: &str) -> u64 {
    let mut memo = HashMap::new();

    recurse_paths(graph, &mut memo, from, to)
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

/// Transposes a graph, i.e. reverses all edges.
/// Not needed
#[allow(dead_code)]
fn transpose_graph<'a>(
    graph: &'a HashMap<&'a str, Vec<&'a str>>,
) -> HashMap<&'a str, Vec<&'a str>> {
    let mut out: HashMap<&str, Vec<&str>> = HashMap::with_capacity(graph.capacity());

    for (&key, outs) in graph {
        for &o in outs {
            out.entry(o).or_default().push(key);
        }
    }

    out.shrink_to_fit();

    out
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

    // gold specific nodes
    out.push_str("svr [shape=diamond, style=filled, fillcolor=green]\n");
    out.push_str("dac [shape=diamond, style=filled, fillcolor=green]\n");
    out.push_str("fft [shape=diamond, style=filled, fillcolor=green]\n");

    out.push_str("}");

    out
}

fn main() -> io::Result<()> {
    let input = read_input_from_env()?;
    let graph = parse(&input);

    // let transpose = transpose_graph(&graph);
    // println!("{}", to_graphviz(&graph));
    // println!("{}", to_graphviz(&transpose));

    println!("silver: {}", silver(&graph));
    println!("gold: {}", gold(&graph));

    Ok(())
}
