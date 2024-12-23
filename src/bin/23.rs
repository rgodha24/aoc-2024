advent_of_code::solution!(23);
use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let mut edges: HashMap<&str, Vec<&str>> = Default::default();
    let mut nodes = HashSet::new();
    for line in input.lines() {
        let (l, r) = line.split_once("-").unwrap();
        edges.entry(l).or_default().push(r);
        edges.entry(r).or_default().push(l);
        nodes.insert(l);
        nodes.insert(r);
    }

    Some(
        nodes
            .iter()
            .tuple_combinations()
            .filter(|(a, b, c)| {
                edges[*a].contains(b) && edges[*a].contains(c) && edges[*b].contains(c)
            })
            .filter(|(a, b, c)| a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let mut graph: HashMap<&str, Vec<&str>> = Default::default();
    let mut nodes = BTreeSet::new();
    for line in input.lines() {
        let (l, r) = line.split_once("-").unwrap();
        graph.entry(l).or_default().push(r);
        graph.entry(r).or_default().push(l);
        nodes.insert(l);
        nodes.insert(r);
    }
    Some(
        bron_kerbosch(Default::default(), nodes, Default::default(), &graph)
            .into_iter()
            .max_by_key(|set| set.len())
            .unwrap()
            .into_iter()
            .join(","),
    )
}

fn bron_kerbosch<'a>(
    r: BTreeSet<&'a str>,
    mut p: BTreeSet<&'a str>,
    mut x: BTreeSet<&'a str>,
    graph: &HashMap<&'a str, Vec<&'a str>>,
) -> HashSet<BTreeSet<&'a str>> {
    let mut cliques = HashSet::new();
    if p.len() == 0 && x.len() == 0 {
        cliques.insert(r.clone());
    }
    for v in p.clone() {
        let mut new_r = r.clone();
        new_r.insert(v);
        let new_p: BTreeSet<_> = p.iter().filter(|x| graph[v].contains(x)).cloned().collect();
        let new_x: BTreeSet<_> = x.iter().filter(|x| graph[v].contains(x)).cloned().collect();
        cliques.extend(bron_kerbosch(new_r, new_p, new_x, graph));
        p.remove(v);
        x.insert(v);
    }

    cliques
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
