use std::collections::{BTreeMap, VecDeque};

#[tracing::instrument]
pub fn process(input: &'static str) -> miette::Result<String> {
    let mut result = 0;
    let graph = parse_graph(input);
    let mut queue = VecDeque::new();
    queue.push_back(NextNode {
        next_node: "svr",
        fft_seen: false,
        dac_seen: false,
    });
    while let Some(mut next) = queue.pop_front() {
        if next.next_node == "out" {
            if next.dac_seen && next.fft_seen {
                result += 1;
            }
            continue;
        }
        if next.next_node == "fft" {
            next.fft_seen = true;
        }
        if next.next_node == "dac" {
            next.dac_seen = true;
        }
        for &neighbour in graph.nodes.get(next.next_node).unwrap().iter() {
            queue.push_back(NextNode {
                next_node: neighbour,
                ..next
            });
        }
    }
    Ok(result.to_string())
}

fn parse_graph(input: &'static str) -> Graph {
    let mut result = Graph::default();
    for line in input.lines() {
        let (node, neighbours) = line.split_once(":").unwrap();
        let is_new = result
            .nodes
            .insert(node, neighbours.split_whitespace().collect())
            .is_none();
        debug_assert!(is_new);
    }
    result
}

#[derive(Debug, Clone, Copy)]
struct NextNode {
    next_node: &'static str,
    fft_seen: bool,
    dac_seen: bool,
}

#[derive(Debug, Default)]
struct Graph {
    nodes: BTreeMap<&'static str, Vec<&'static str>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";
        assert_eq!(process(input)?, "2");
        Ok(())
    }
}
