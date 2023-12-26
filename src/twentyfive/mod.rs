#![allow(warnings, unused)]

use std::collections::HashSet;
use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use petgraph::dot::Dot;
use petgraph::graphmap::UnGraphMap;
use rustworkx_core::petgraph;
use rustworkx_core::connectivity::stoer_wagner_min_cut;


pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    // let part2 = process(input, false);
    // println!("Part2: {}", part2.to_string());
}

fn process(input: &str, part2: bool) -> usize {
    let mut edges = HashSet::<(&str, &str)>::new();

    input.lines()
        .filter_map(|l| l.split_once(':'))
        .for_each(|(node, s)| s.split_whitespace()
            .for_each(|neighbor| {
                edges.insert((node, neighbor));
            }));

    let graph = UnGraphMap::<&str, u32>::from_edges(edges);

    // Write to file
    let file_path = PathBuf::from(env::current_dir().unwrap()).join("src/twentyfive").join("graph.txt");
    fs::write(&file_path, format!("{}", Dot::new(&graph))).expect("fail");

    let Ok(Some((_, group))) = stoer_wagner_min_cut(&graph, |_| Ok::<_, ()>(1))
        else { panic!() };
    let g1 = group.len();
    let g2 = graph.node_count() - g1;

    eprintln!("1st group: {g1}");
    eprintln!("2nd group: {g2}");

    g1 * g2
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn part1() {
        let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        assert_eq!(54, process(input, false));
    }
}
