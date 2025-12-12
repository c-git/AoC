use std::collections::{BTreeMap, HashMap};

use miette::Context;

#[tracing::instrument]
pub fn process(input: &'static str) -> miette::Result<String> {
    let graph = parse_graph(input);
    let result = dfs(
        &graph,
        NextNode {
            next_node: "svr",
            fft_seen: false,
            dac_seen: false,
        },
        &mut HashMap::new(),
    );
    Ok(result.to_string())
}

fn dfs(graph: &Graph, start: NextNode, memo: &mut HashMap<NextNode, usize>) -> usize {
    if let Some(result) = memo.get(&start) {
        return *result;
    }
    let mut result = 0;
    if start.next_node == "out" {
        if start.dac_seen && start.fft_seen {
            return 1;
        }
        return 0;
    }
    let mut next = start;
    if next.next_node == "fft" {
        next.fft_seen = true;
    }
    if next.next_node == "dac" {
        next.dac_seen = true;
    }

    for &neighbour in graph
        .nodes
        .get(next.next_node)
        .wrap_err_with(|| format!("unable to find: {:?}", next.next_node))
        .unwrap()
        .iter()
    {
        result += dfs(
            graph,
            NextNode {
                next_node: neighbour,
                ..next
            },
            memo,
        );
    }
    memo.insert(start, result);
    result
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
