use core::f64;
use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

use miette::Context;

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

pub fn get_nearest_neighbours(
    points: &[JunctionBox],
) -> BinaryHeap<Reverse<NearestNeighboursInfo>> {
    let mut result = BinaryHeap::new();
    for (i1, p1) in points.iter().enumerate() {
        for (i2, p2) in points.iter().enumerate() {
            if i1 != i2 {
                let junction_box_indices = if i1 < i2 { [i1, i2] } else { [i2, i1] };
                let distance = p1.distance_to(p2);
                let info = NearestNeighboursInfo {
                    distance,
                    junction_box_indices,
                };
                result.push(Reverse(info));
            }
        }
    }
    result
}

pub struct UnionFind {
    roots: Vec<usize>,
    group_size: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            roots: (0..size).collect(),
            group_size: vec![1; size],
        }
    }

    pub fn join(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }
        if self.group_size[root_x] > self.group_size[root_y] {
            self.roots[root_y] = root_x;
            self.group_size[root_x] += self.group_size[root_y];
        } else {
            self.roots[root_x] = root_y;
            self.group_size[root_y] += self.group_size[root_x];
        }
    }

    pub fn join_pair(&mut self, pair: [usize; 2]) {
        self.join(pair[0], pair[1]);
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.roots[x] != x {
            self.roots[x] = self.find(self.roots[x]);
        }
        self.roots[x]
    }

    pub fn group_size(&mut self, x: usize) -> usize {
        let x_root = self.find(x);
        self.group_size[x_root]
    }
}

pub struct JunctionBox {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
impl JunctionBox {
    fn distance_to(&self, other_point: &JunctionBox) -> f64 {
        ((self.x as f64 - other_point.x as f64).powi(2)
            + (self.y as f64 - other_point.y as f64).powi(2)
            + (self.z as f64 - other_point.z as f64).powi(2))
        .sqrt()
    }
}

#[derive(Debug)]
pub struct NearestNeighboursInfo {
    distance: f64,
    // pair of indices with lower index number first
    pub junction_box_indices: [usize; 2],
}

impl Default for NearestNeighboursInfo {
    fn default() -> Self {
        Self {
            distance: f64::MAX,
            junction_box_indices: Default::default(),
        }
    }
}

impl PartialEq for NearestNeighboursInfo {
    fn eq(&self, other: &Self) -> bool {
        debug_assert!(self.junction_box_indices[0] < self.junction_box_indices[1]);
        debug_assert!(other.junction_box_indices[0] < other.junction_box_indices[1]);
        debug_assert!(
            self.junction_box_indices != other.junction_box_indices
                || self.distance == other.distance
        );
        self.junction_box_indices == other.junction_box_indices
    }
}

impl Eq for NearestNeighboursInfo {}

impl PartialOrd for NearestNeighboursInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NearestNeighboursInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .expect("all f64 used are expected to be comparable")
    }
}

impl FromIterator<i64> for JunctionBox {
    fn from_iter<T: IntoIterator<Item = i64>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let x = iter.next().expect("failed to get first value from row");
        let y = iter.next().expect("failed to get second value from row");
        let z = iter.next().expect("failed to get third value from row");
        debug_assert!(iter.next().is_none(), "extra info found on line");
        Self { x, y, z }
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
        assert_eq!(process(input)?, "40");
        Ok(())
    }
}
