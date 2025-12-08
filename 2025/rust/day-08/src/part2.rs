use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

use miette::Context;

use crate::part1::{JunctionBox, NearestNeighboursInfo, UnionFind, get_nearest_neighbours};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    const NUM_LARGEST_TO_CONSIDER: usize = 3;
    const BOXES_TO_CONNECT: usize = if cfg!(debug_assertions) { 10 } else { 1000 };
    dbg!(BOXES_TO_CONNECT);

    // Convert input into points
    let points: Vec<JunctionBox> = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|value| value.parse::<i64>().expect("failed to convert number"))
                .collect()
        })
        .collect();

    // Find nearest connections
    let mut nearest_neighbours = get_nearest_neighbours(&points);

    // Join and track with union find
    let mut links: BTreeSet<NearestNeighboursInfo> = BTreeSet::new();
    let mut union_find = UnionFind::new(points.len());
    while links.len() < BOXES_TO_CONNECT {
        let next_candidate = nearest_neighbours
            .pop()
            .map(|Reverse(x)| x)
            .wrap_err("out of connections before meeting quota")?;
        if links.contains(&next_candidate) {
            // Already linked
            continue;
        }
        union_find.join_pair(next_candidate.junction_box_indices);
        links.insert(next_candidate);
    }

    // Calculate output
    let mut largest = BinaryHeap::new();
    let mut seen_root = BTreeSet::new();
    for index in 0..points.len() {
        let root = union_find.find(index);
        if seen_root.contains(&root) {
            // Already processed this root
            continue;
        }
        seen_root.insert(root);
        largest.push(Reverse(union_find.group_size(root)));
        if largest.len() > NUM_LARGEST_TO_CONSIDER {
            largest.pop();
        }
        debug_assert!(
            largest.len() <= NUM_LARGEST_TO_CONSIDER,
            "we should have just reduced to make this true"
        );
    }

    Ok(largest
        .into_iter()
        .fold(1, |acc, Reverse(x)| acc * x)
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
        assert_eq!(process(input)?, "40");
        Ok(())
    }
}
