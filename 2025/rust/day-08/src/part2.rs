use std::{cmp::Reverse, collections::BTreeSet};

use miette::Context;

use crate::part1::{JunctionBox, NearestNeighboursInfo, UnionFind, get_nearest_neighbours};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
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
    loop {
        let next_candidate = nearest_neighbours
            .pop()
            .map(|Reverse(x)| x)
            .wrap_err("out of connections before meeting quota")?;
        if links.contains(&next_candidate) {
            // Already linked
            continue;
        }
        union_find.join_pair(next_candidate.junction_box_indices);
        if union_find.group_size(next_candidate.junction_box_indices[0]) == points.len() {
            // All points are now connected
            let x1 = points[next_candidate.junction_box_indices[0]].x;
            let x2 = points[next_candidate.junction_box_indices[1]].x;
            return Ok((x1 * x2).to_string());
        }
        links.insert(next_candidate);
    }
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
        assert_eq!(process(input)?, "25272");
        Ok(())
    }
}
