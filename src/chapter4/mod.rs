pub mod bst_sequences;
pub mod build_order;
pub mod check_balanced;
pub mod check_subtree;
pub mod first_common_ancestor;
pub mod list_of_depths;
pub mod min_tree;
pub mod paths_with_sum;
pub mod random_node;
pub mod route_between_nodes;
pub mod successor;
pub mod validate_bst;

use slotmap::{new_key_type, SlotMap};
use std::cmp;

new_key_type! { pub struct GraphKey; }
/// A graph node for an adjacency list graph structure.
pub struct GraphNode<T> {
    pub data: T,
    pub edges: Vec<GraphKey>,
}

pub type GraphNodes<T> = SlotMap<GraphKey, GraphNode<T>>;

/// Builds an adjacency list graph given the node datas and an adjacency matrix
/// that describes the edges between the nodes.
pub fn build_graph<T>(
    data: Vec<T>,
    edges: Vec<Vec<bool>>,
    return_nodes: Vec<usize>,
    nodes: &mut GraphNodes<T>,
) -> Vec<GraphKey> {
    let keys: Vec<GraphKey> = data
        .into_iter()
        .map(|data| {
            nodes.insert(GraphNode {
                data,
                edges: Vec::new(),
            })
        })
        .collect();

    for (i, &node) in keys.iter().enumerate() {
        for (j, &edge) in edges[i].iter().enumerate() {
            if edge {
                nodes[node].edges.push(keys[j]);
            }
        }
    }

    keys.into_iter()
        .enumerate()
        .filter(|&(i, _)| return_nodes.contains(&i))
        .map(|(_, n)| n)
        .collect()
}

#[derive(PartialEq, Debug, Clone)]
pub struct Tree<T> {
    data: T,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
}

pub fn tree_height<T>(tree: Option<&Box<Tree<T>>>) -> usize {
    if tree.is_none() {
        return 0;
    }

    let left = tree_height(tree.and_then(|n| n.left.as_ref()));
    let right = tree_height(tree.and_then(|n| n.right.as_ref()));

    cmp::max(left, right) + 1
}
