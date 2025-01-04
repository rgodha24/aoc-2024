advent_of_code::solution!(23);
use core::str;
use std::collections::{BTreeSet, HashSet};

use itertools::Itertools;
use rustc_hash::FxHashMap;

fn parse(input: &str) -> FxHashMap<u16, Vec<u16>> {
    let mut edges: FxHashMap<u16, Vec<u16>> = Default::default();
    for line in input.lines() {
        let line = line.as_bytes();
        let l = u16::from_ne_bytes(line[..2].try_into().unwrap());
        let r = u16::from_ne_bytes(line[3..5].try_into().unwrap());
        edges.entry(l).or_default().push(r);
        edges.entry(r).or_default().push(l);
    }

    edges
}

pub fn part_one(input: &str) -> Option<usize> {
    let edges = parse(input);

    Some(
        edges
            .keys()
            .tuple_combinations()
            .filter(|(a, b, c)| {
                edges[*a].contains(b) && edges[*a].contains(c) && edges[*b].contains(c)
            })
            .filter(|(&a, &b, &c)| {
                (a.to_ne_bytes()[0]) == b't'
                    || (b.to_ne_bytes()[0]) == b't'
                    || (c.to_ne_bytes()[0]) == b't'
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let edges = parse(input);

    Some(
        bron_kerbosch(
            Default::default(),
            edges.keys().cloned().collect(),
            Default::default(),
            &edges,
        )
        .into_iter()
        .max_by_key(|set| set.len())
        .unwrap()
        .into_iter()
        .map(|n| unsafe { str::from_utf8_unchecked(&n.to_ne_bytes()) }.to_owned())
        .sorted()
        .join(","),
    )
}

fn bron_kerbosch<'a>(
    r: BTreeSet<u16>,
    mut p: BTreeSet<u16>,
    mut x: BTreeSet<u16>,
    graph: &FxHashMap<u16, Vec<u16>>,
) -> HashSet<BTreeSet<u16>> {
    let mut cliques = HashSet::new();
    if p.len() == 0 && x.len() == 0 {
        cliques.insert(r.clone());
    }
    for v in p.clone() {
        let mut new_r = r.clone();
        new_r.insert(v);
        let new_p: BTreeSet<_> = p
            .iter()
            .filter(|x| graph[&v].contains(x))
            .cloned()
            .collect();
        let new_x: BTreeSet<_> = x
            .iter()
            .filter(|x| graph[&v].contains(x))
            .cloned()
            .collect();
        cliques.extend(bron_kerbosch(new_r, new_p, new_x, graph));
        p.remove(&v);
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
