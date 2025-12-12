use std::collections::{BTreeMap, VecDeque};

#[tracing::instrument]
pub fn process(input: &'static str) -> miette::Result<String> {
    let mut result = 0;
    let graph = parse_graph(input);
    let mut queue = VecDeque::new();
    queue.push_back("you");
    while let Some(next) = queue.pop_front() {
        if next == "out" {
            result += 1;
            continue;
        }
        for &neighbour in graph.nodes.get(next).unwrap().iter() {
            queue.push_back(neighbour);
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

#[derive(Debug, Default)]
struct Graph {
    nodes: BTreeMap<&'static str, Vec<&'static str>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";
        assert_eq!(process(input)?, "5");
        Ok(())
    }
}
