pub mod route_between_nodes;
pub mod min_tree;
pub mod list_of_depths;
pub mod check_balanced;
pub mod validate_bst;
pub mod successor;
pub mod build_order;
pub mod first_common_ancestor;

use std::mem;
use std::cmp;

/// A graph node for an adjacency list graph structure.
pub struct Node<T> {
    data: T,
    edges: Vec<*mut Node<T>>,
}

impl<T> Node<T> {
    pub unsafe fn new(data: T) -> *mut Node<T> {
        let node = Box::new(Node {
            data: data,
            edges: Vec::new(),
        });

        mem::transmute::<Box<Node<T>>, *mut Node<T>>(node)
    }
}

/// A tree node that also contains a pointer to the parent node.
pub struct TreeNode<T> {
    data: T,
    left: Option<*mut TreeNode<T>>,
    right: Option<*mut TreeNode<T>>,
    parent: Option<*mut TreeNode<T>>,
}

impl<T> TreeNode<T> {
    pub unsafe fn new(data: T) -> *mut TreeNode<T> {
        let node = Box::new(TreeNode {
            data: data,
            left: None,
            right: None,
            parent: None,
        });

        mem::transmute::<Box<TreeNode<T>>, *mut TreeNode<T>>(node)
    }

    pub unsafe fn new_with_parent(data: T,
                                  parent: Option<*mut TreeNode<T>>,
                                  left: bool)
                                  -> *mut TreeNode<T> {
        let node = Box::new(TreeNode {
            data: data,
            left: None,
            right: None,
            parent: parent,
        });
        let node_ptr = mem::transmute::<Box<TreeNode<T>>, *mut TreeNode<T>>(node);

        // Link parent's left or right pointer to the node.
        parent.map(|p| {
            if left {
                (*p).left = Some(node_ptr);
            } else {
                (*p).right = Some(node_ptr);
            }
        });

        node_ptr
    }
}

pub unsafe fn free_tree<T>(tree: *mut TreeNode<T>) {
    if let Some(left) = (*tree).left {
        free_tree(left);
    }
    if let Some(right) = (*tree).right {
        free_tree(right);
    }

    // Free the node.
    mem::transmute::<*mut TreeNode<T>, Box<TreeNode<T>>>(tree);
}

/// Builds an adjacency list graph given the node datas and an adjacency matrix
/// that describes the edges between the nodes.
pub unsafe fn build_graph<T>(node_data: Vec<T>,
                             node_edges: Vec<Vec<bool>>,
                             return_nodes: Vec<usize>)
                             -> Vec<*mut Node<T>> {
    let nodes: Vec<_> = node_data.into_iter().map(|data| Node::new(data)).collect();
    for (i, &node) in nodes.iter().enumerate() {
        for (j, edge) in node_edges[i].iter().enumerate() {
            if *edge {
                (*node).edges.push(nodes[j]);
            }
        }
    }

    nodes.into_iter()
        .enumerate()
        .filter(|&(i, _)| return_nodes.contains(&i))
        .map(|(_, n)| n)
        .collect()
}

pub unsafe fn free_graph<T>(n: *mut Node<T>) {
    if !(*n).edges.is_empty() {
        for edge in (*n).edges.iter() {
            free_graph(*edge);
        }
    }

    mem::transmute::<*mut Node<T>, Box<Node<T>>>(n);
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
