use super::parse_lines;

use std::collections::{HashSet, HashMap};

use petgraph::prelude::NodeIndex;
use petgraph::{Graph, Undirected, algo};

fn parse_input(input: &str) -> Vec<(String, String)> {
    let parse_fn = |s: &str| -> (String, String) {
        let parts: Vec<&str> = s.split(')').collect();

        (parts[0].to_string(), parts[1].to_string())
    };

    parse_lines(input.trim_end_matches('\n'), parse_fn)
}

fn construct_graph(orb_relationships: &Vec<(String, String)>) -> (HashMap<&str, NodeIndex<u32>>, Graph<&str, usize, Undirected>) {
    let nodes = orb_relationships
        .iter()
        .map(|(c, _)| c.as_str())
        .chain(orb_relationships
            .iter()
            .map(|(_, o)| o.as_str())
        )
        .collect::<HashSet<&str>>();

    let mut g = Graph::<&str, usize, Undirected>::default();

    let indexes: HashMap<&str, NodeIndex> = nodes
        .iter()
        .map(|n| (*n, g.add_node(*n)))
        .collect();

    for (center, orbit) in orb_relationships {
        g.add_edge(*indexes.get(orbit.as_str()).unwrap(), *indexes.get(center.as_str()).unwrap(), 1);
    }

    (indexes, g)
}

pub fn run(input_str: &String) {
    println!("\n-- Day 6 --");

    let input = parse_input(input_str);
    let test_input = parse_input("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");

    let dist = |g: &Graph<&str, usize, Undirected>, start: NodeIndex<u32>, end: NodeIndex<u32>| -> Option<usize> {
        match algo::astar(&g, start, |n| n == end, |e| *e.weight(), |_| 0) {
            Some((cost, _)) => Some(cost),
            None => None,
        }
    };

    // Part 1
    let count_orbits = |input: &Vec<(String, String)>| -> usize {
        let (idx_map, g) = construct_graph(input);

        idx_map.keys().into_iter().map(|n| dist(&g, idx_map[n], idx_map["COM"]).unwrap()).sum()
    };

    assert_eq!(count_orbits(&test_input), 42);

    println!("Part 1: {}", count_orbits(&input));

    // Part 2
    let orbit_transfers = |input: &Vec<(String, String)>, start: &str, end: &str| -> usize {
        let (idx_map, g) = construct_graph(input);

        dist(&g, idx_map[start], idx_map[end]).unwrap() - 2
    };

    assert_eq!(orbit_transfers(&test_input, "L", "F"), 2);
    assert_eq!(orbit_transfers(&test_input, "I", "H"), 3);
    assert_eq!(orbit_transfers(&test_input, "I", "L"), 3);

    println!("Part 1: {}", orbit_transfers(&input, "YOU", "SAN"));
}
