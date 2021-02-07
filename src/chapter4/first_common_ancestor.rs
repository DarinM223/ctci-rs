use super::Tree;
use slotmap::{new_key_type, SlotMap};

new_key_type! { pub struct TreeKey; }
impl TreeKey {
    pub fn left<T>(self, nodes: &SlotMap<TreeKey, TreeNode<T>>) -> Option<TreeKey> {
        nodes.get(self).and_then(|n| n.left)
    }

    pub fn right<T>(self, nodes: &SlotMap<TreeKey, TreeNode<T>>) -> Option<TreeKey> {
        nodes.get(self).and_then(|n| n.right)
    }

    pub fn parent<T>(self, nodes: &SlotMap<TreeKey, TreeNode<T>>) -> Option<TreeKey> {
        nodes.get(self).and_then(|n| n.parent)
    }

    pub fn new<T>(data: T, nodes: &mut SlotMap<TreeKey, TreeNode<T>>) -> TreeKey {
        let node = TreeNode {
            data,
            left: None,
            right: None,
            parent: None,
        };
        nodes.insert(node)
    }

    pub fn new_with_parent<T>(
        data: T,
        parent: TreeKey,
        left: bool,
        nodes: &mut SlotMap<TreeKey, TreeNode<T>>,
    ) -> TreeKey {
        let node = TreeNode {
            data,
            left: None,
            right: None,
            parent: Some(parent),
        };
        let node_key = nodes.insert(node);
        if left {
            nodes[parent].left = Some(node_key);
        } else {
            nodes[parent].right = Some(node_key);
        }
        node_key
    }
}

pub struct TreeNode<T> {
    pub data: T,
    pub left: Option<TreeKey>,
    pub right: Option<TreeKey>,
    pub parent: Option<TreeKey>,
}

/// Solution if the tree nodes are allowed access to the parent.
/// The solution is similar to the intersection problem in chapter 2 except that the linked lists
/// are now the tree nodes with the parent pointers.
pub fn common_ancestor_parent<T>(
    node1: TreeKey,
    node2: TreeKey,
    nodes: &SlotMap<TreeKey, TreeNode<T>>,
) -> Option<TreeKey> {
    let n1_size = depth(node1, nodes);
    let n2_size = depth(node2, nodes);
    let (mut bigger, mut smaller, mut difference) = if n1_size > n2_size {
        (Some(node1), Some(node2), n1_size - n2_size)
    } else {
        (Some(node2), Some(node1), n2_size - n1_size)
    };

    while difference > 0 {
        bigger = bigger.and_then(|n| n.parent(nodes));
        difference -= 1;
    }

    while let (Some(bn), Some(sn)) = (bigger, smaller) {
        if bn == sn {
            return Some(bn);
        }

        bigger = bn.parent(nodes);
        smaller = sn.parent(nodes);
    }

    None
}

fn depth<T>(node: TreeKey, nodes: &SlotMap<TreeKey, TreeNode<T>>) -> usize {
    let mut depth = 0;
    let mut curr = Some(node);
    while let Some(node) = curr {
        depth += 1;
        curr = node.parent(nodes);
    }
    depth
}

/// A node that contains both data and an id (in order to do comparisons).
type Node<'a, T> = Option<&'a Box<Tree<(u32, T)>>>;

/// Solution if the tree nodes are not allowed access to the parent.
pub fn common_ancestor<'a, T>(
    tree: &'a Box<Tree<(u32, T)>>,
    node1: u32,
    node2: u32,
) -> Option<&'a Box<Tree<(u32, T)>>> {
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
        let mut nodes = SlotMap::with_key();
        let tree = TreeKey::new(b'a', &mut nodes);
        let bnode = TreeKey::new_with_parent(b'b', tree, true, &mut nodes);
        let cnode = TreeKey::new_with_parent(b'c', bnode, true, &mut nodes);
        let dnode = TreeKey::new_with_parent(b'd', bnode, false, &mut nodes);
        let enode = TreeKey::new_with_parent(b'e', tree, false, &mut nodes);
        let fnode = TreeKey::new_with_parent(b'f', enode, true, &mut nodes);
        let _ = TreeKey::new_with_parent(b'g', enode, false, &mut nodes);

        assert_eq!(common_ancestor_parent(cnode, dnode, &nodes), Some(bnode));
        assert_eq!(common_ancestor_parent(cnode, cnode, &nodes), Some(cnode));
        assert_eq!(common_ancestor_parent(cnode, fnode, &nodes), Some(tree));
        assert_eq!(common_ancestor_parent(bnode, enode, &nodes), Some(tree));
    }
}
