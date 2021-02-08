use super::{GraphKey, GraphNodes};
use std::collections::{HashSet, VecDeque};

pub fn route_between_nodes<T>(node1: GraphKey, node2: GraphKey, nodes: &GraphNodes<T>) -> bool {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(node1);
    while let Some(node) = queue.pop_front() {
        if !visited.contains(&node) {
            if node == node2 {
                return true;
            }
            for edge in nodes[node].edges.iter() {
                queue.push_back(*edge);
            }
            visited.insert(node);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use slotmap::SlotMap;

    use super::super::build_graph;
    use super::*;

    #[test]
    fn test_route_between_nodes() {
        let mut nodes = SlotMap::with_key();
        let keys = build_graph(
            vec![1, 2, 3],
            vec![
                vec![false, false, true],
                vec![false, false, true],
                vec![false, false, false],
            ],
            vec![0, 1, 2],
            &mut nodes,
        );
        assert_eq!(route_between_nodes(keys[0], keys[2], &nodes), true);
        assert_eq!(route_between_nodes(keys[1], keys[2], &nodes), true);
        assert_eq!(route_between_nodes(keys[0], keys[1], &nodes), false);
    }
}
