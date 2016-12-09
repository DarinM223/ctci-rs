use std::collections::VecDeque;
use super::Node;

pub unsafe fn route_between_nodes<T>(node1: *mut Node<T>, node2: *mut Node<T>) -> bool {
    let mut queue = VecDeque::new();

    queue.push_back(node1);
    while let Some(node) = queue.pop_front() {
        if !(*node).visited {
            if node == node2 {
                return true;
            }
            for edge in (*node).edges.iter() {
                queue.push_back(*edge);
            }
            (*node).visited = true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::super::build_graph;
    use super::*;

    #[test]
    fn test_route_between_nodes() {
        unsafe {
            let nodes = build_graph(vec![1, 2, 3],
                                    vec![vec![false, false, true],
                                         vec![false, false, true],
                                         vec![false, false, false]],
                                    vec![0, 1, 2]);
            assert_eq!(route_between_nodes(nodes[0], nodes[2]), true);
            assert_eq!(route_between_nodes(nodes[1], nodes[2]), true);
            assert_eq!(route_between_nodes(nodes[0], nodes[1]), false);
        }
    }
}
