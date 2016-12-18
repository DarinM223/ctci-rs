use super::{Tree, TreeNode};

/// Solution if the tree nodes are allowed access to the parent.
/// The solution is similar to the intersection problem in chapter 2 except that the linked lists
/// are now the tree nodes with the parent pointers.
pub unsafe fn common_ancestor_parent<T>(node1: *mut TreeNode<T>,
                                        node2: *mut TreeNode<T>)
                                        -> Option<*mut TreeNode<T>> {
    let n1_size = depth(node1);
    let n2_size = depth(node2);

    let (mut bigger, mut smaller) = if n1_size > n2_size {
        (Some(node1), Some(node2))
    } else {
        (Some(node2), Some(node1))
    };

    let mut difference = (n1_size - n2_size).abs();
    while difference > 0 {
        bigger = bigger.and_then(|n| (*n).parent);
        difference -= 1;
    }

    while let (Some(bn), Some(sn)) = (bigger, smaller) {
        if bn == sn {
            return Some(bn);
        }

        bigger = (*bn).parent;
        smaller = (*sn).parent;
    }

    None
}

unsafe fn depth<T>(node: *mut TreeNode<T>) -> i32 {
    let mut depth = 0;
    let mut curr = Some(node);

    while let Some(n) = curr {
        depth += 1;
        curr = (*n).parent;
    }

    depth
}

/// A node that contains both data and an id (in order to do comparisons).
type Node<'a, T> = Option<&'a Box<Tree<(u32, T)>>>;

/// Solution if the tree nodes are not allowed access to the parent.
pub fn common_ancestor<'a, T>(tree: &'a Box<Tree<(u32, T)>>,
                              node1: u32,
                              node2: u32)
                              -> Option<&'a Box<Tree<(u32, T)>>> {
    let (node, finished) = common_ancestor_rec(Some(tree), node1, node2);

    if !finished {
        None
    } else {
        node
    }
}

fn common_ancestor_rec<'a, T>(tree: Node<'a, T>, node1: u32, node2: u32) -> (Node<'a, T>, bool) {
    let node_id = tree.map(|n| n.data.0);
    if node_id.is_none() {
        return (tree, false);
    } else if node_id == Some(node1) && node_id == Some(node2) {
        return (tree, true);
    } else if node_id == Some(node1) || node_id == Some(node2) {
        return (tree, false);
    }

    let (left, finished) = common_ancestor_rec(tree.and_then(|n| n.left.as_ref()), node1, node2);
    if finished {
        return (left, finished);
    }

    let (right, finished) = common_ancestor_rec(tree.and_then(|n| n.right.as_ref()), node1, node2);
    if finished {
        return (right, finished);
    }

    match (left, right) {
        (Some(_), Some(_)) => (tree, true),
        (Some(n), None) => (Some(n), false),
        (None, Some(n)) => (Some(n), false),
        _ => (None, false),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Tree, TreeNode, free_tree};
    use super::*;

    #[test]
    fn test_common_ancestor() {
        let tree = Box::new(Tree {
            data: (1, b'a'),
            left: Some(Box::new(Tree {
                data: (2, b'b'),
                left: Some(Box::new(Tree {
                    data: (4, b'c'),
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Tree {
                    data: (5, b'd'),
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(Tree {
                data: (3, b'e'),
                left: Some(Box::new(Tree {
                    data: (6, b'f'),
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Tree {
                    data: (7, b'g'),
                    left: None,
                    right: None,
                })),
            })),
        });

        assert_eq!(common_ancestor(&tree, 4, 5).map(|n| n.data.0), Some(2));
        assert_eq!(common_ancestor(&tree, 4, 4).map(|n| n.data.0), Some(4));
        assert_eq!(common_ancestor(&tree, 4, 6).map(|n| n.data.0), Some(1));
        assert_eq!(common_ancestor(&tree, 2, 3).map(|n| n.data.0), Some(1));
        assert_eq!(common_ancestor(&tree, 4, 9).map(|n| n.data.0), None);
    }

    #[test]
    fn test_common_ancestor_parent() {
        unsafe {
            let tree = TreeNode::new(b'a');
            let bnode = TreeNode::new_with_parent(b'b', Some(tree), true);
            let cnode = TreeNode::new_with_parent(b'c', Some(bnode), true);
            let dnode = TreeNode::new_with_parent(b'd', Some(bnode), false);
            let enode = TreeNode::new_with_parent(b'e', Some(tree), false);
            let fnode = TreeNode::new_with_parent(b'f', Some(enode), true);
            let _ = TreeNode::new_with_parent(b'g', Some(enode), false);

            assert_eq!(common_ancestor_parent(cnode, dnode), Some(bnode));
            assert_eq!(common_ancestor_parent(cnode, cnode), Some(cnode));
            assert_eq!(common_ancestor_parent(cnode, fnode), Some(tree));
            assert_eq!(common_ancestor_parent(bnode, enode), Some(tree));

            free_tree(tree);
        }
    }
}
